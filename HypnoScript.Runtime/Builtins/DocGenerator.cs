using System;
using System.IO;
using System.Linq;
using System.Reflection;
using System.Text;
using System.Xml.Linq;

namespace HypnoScript.Runtime.Builtins
{
    /// <summary>
    /// Simple documentation generator for Builtins. Scans all Builtins/*.cs files and generates Markdown docs.
    /// </summary>
    public static class DocGenerator
    {
        private static XDocument? _xmlDocCache = null;
        private static string? _xmlDocPathCache = null;

        public static void GenerateMarkdownDocs(string outputDir)
        {
            var builtinsDir = Path.GetDirectoryName(typeof(DocGenerator).Assembly.Location);
            var builtinTypes = Assembly.GetExecutingAssembly().GetTypes()
                .Where(t => t.IsClass && t.IsPublic && t.Namespace == "HypnoScript.Runtime.Builtins")
                .ToList();
            foreach (var type in builtinTypes)
            {
                var sb = new StringBuilder();
                sb.AppendLine($"# {type.Name.Replace("Builtins", " Functions")}");
                sb.AppendLine();
                foreach (var method in type.GetMethods(BindingFlags.Public | BindingFlags.Static))
                {
                    sb.AppendLine($"## {method.Name}");
                    sb.AppendLine();
                    sb.AppendLine($"**Signature:** `{method}`");
                    sb.AppendLine();
                    // Try to get XML doc comment (if available)
                    var xmlComment = GetXmlDocComment(type, method);
                    if (!string.IsNullOrWhiteSpace(xmlComment))
                        sb.AppendLine(xmlComment);
                    else
                        sb.AppendLine($"_No description available._");
                    sb.AppendLine();
                }
                var outFile = Path.Combine(outputDir, $"{type.Name.Replace("Builtins", "").ToLowerInvariant()}-functions.md");
                File.WriteAllText(outFile, sb.ToString());
            }
        }

        private static string? GetXmlDocComment(Type type, System.Reflection.MethodInfo method)
        {
            // Ermittle den Pfad zur XML-Dokumentationsdatei (im gleichen Verzeichnis wie die DLL)
            var asm = type.Assembly;
            var asmLocation = asm.Location;
            var xmlPath = Path.ChangeExtension(asmLocation, ".xml");
            if (!File.Exists(xmlPath))
                return null;

            // Cache das XML-Dokument für Performance
            if (_xmlDocCache == null || _xmlDocPathCache != xmlPath)
            {
                _xmlDocCache = XDocument.Load(xmlPath);
                _xmlDocPathCache = xmlPath;
            }
            var xml = _xmlDocCache;
            if (xml == null) return null;

            // Erzeuge den Member-Name wie in der XML-Doku (z.B. M:Namespace.Type.Method(ParamType,ParamType))
            string memberName = "M:" + type.FullName + "." + method.Name;
            var parameters = method.GetParameters();
            if (parameters.Length > 0)
            {
                memberName += "(" + string.Join(",", parameters.Select(p => GetXmlTypeName(p.ParameterType))) + ")";
            }
            // Suche das passende member-Element
            var member = xml.Descendants("member").FirstOrDefault(m => (string?)m.Attribute("name") == memberName);
            if (member == null)
                return null;
            // Hole den <summary>-Text
            var summary = member.Element("summary")?.Value?.Trim();
            return summary;
        }

        // Hilfsfunktion: .NET-Typnamen zu XML-Doc-Typnamen
        private static string GetXmlTypeName(Type t)
        {
            if (t.IsGenericType)
            {
                var genericType = t.GetGenericTypeDefinition();
                var genericArgs = t.GetGenericArguments();
                var baseName = genericType.FullName?.Split('`')[0];
                return baseName + "{" + string.Join(",", genericArgs.Select(GetXmlTypeName)) + "}";
            }
            if (t.IsArray)
                return GetXmlTypeName(t.GetElementType()!) + "[]";
            return t.FullName ?? t.Name;
        }
    }
}
