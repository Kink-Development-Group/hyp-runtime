using System;
using System.IO;
using System.Text;
using System.Collections.Generic;
using System.Linq;
using HypnoScript.CLI;
using HypnoScript.LexerParser.Lexer;
using HypnoScript.LexerParser.Parser;
using HypnoScript.LexerParser.AST;

namespace HypnoScript.CLI.Commands
{
    public static class DocsCommand
    {
        public static int Execute(string filePath, bool debug, bool verbose)
        {
            AppLogger.Info("=== DOCS MODE ===");
            AppLogger.Info("📚 Generating HypnoScript Documentation...");

            if (!File.Exists(filePath))
            {
                AppLogger.Error($"File not found: {filePath}");
                return 2;
            }

            try
            {
                // Datei parsen
                string source = File.ReadAllText(filePath);
                var lexer = new HypnoLexer(source);
                var tokens = lexer.Lex().ToList();
                var parser = new HypnoParser(tokens);
                var program = parser.ParseProgram();

                // Dokumentation generieren
                var documentation = GenerateDocumentation(program, filePath, debug, verbose);

                // Ausgabedateien erstellen
                var outputDir = Path.Combine(Path.GetDirectoryName(filePath) ?? ".", "docs");
                Directory.CreateDirectory(outputDir);

                // HTML-Dokumentation
                var htmlDoc = GenerateHtmlDocumentation(documentation);
                var htmlPath = Path.Combine(outputDir, Path.GetFileNameWithoutExtension(filePath) + "_docs.html");
                File.WriteAllText(htmlPath, htmlDoc, Encoding.UTF8);

                // Markdown-Dokumentation
                var markdownDoc = GenerateMarkdownDocumentation(documentation);
                var markdownPath = Path.Combine(outputDir, Path.GetFileNameWithoutExtension(filePath) + "_docs.md");
                File.WriteAllText(markdownPath, markdownDoc, Encoding.UTF8);

                // JSON-Dokumentation
                var jsonDoc = GenerateJsonDocumentation(documentation);
                var jsonPath = Path.Combine(outputDir, Path.GetFileNameWithoutExtension(filePath) + "_docs.json");
                File.WriteAllText(jsonPath, jsonDoc, Encoding.UTF8);

                AppLogger.Info("✅ Documentation generated successfully!");
                AppLogger.Info($"📄 HTML: {htmlPath}");
                AppLogger.Info($"📄 Markdown: {markdownPath}");
                AppLogger.Info($"📄 JSON: {jsonPath}");

                if (verbose)
                {
                    AppLogger.Info("\n📊 Documentation Summary:");
                    AppLogger.Info($"  - Functions: {documentation.Functions.Count}");
                    AppLogger.Info($"  - Variables: {documentation.Variables.Count}");
                    AppLogger.Info($"  - Sessions: {documentation.Sessions.Count}");
                    AppLogger.Info($"  - Tranceifies: {documentation.Tranceifies.Count}");
                    AppLogger.Info($"  - Lines of Code: {documentation.LinesOfCode}");
                }

                return 0;
            }
            catch (Exception ex)
            {
                AppLogger.Error($"Documentation generation failed for {filePath}", ex);
                return 1;
            }
        }

        private static DocumentationData GenerateDocumentation(ProgramNode program, string filePath, bool debug, bool verbose)
        {
            var doc = new DocumentationData
            {
                FileName = Path.GetFileName(filePath),
                FilePath = filePath,
                GeneratedAt = DateTime.Now,
                LinesOfCode = File.ReadAllLines(filePath).Length
            };

            // Analysiere alle Statements
            foreach (var stmt in program.Statements)
            {
                AnalyzeStatement(stmt, doc, debug);
            }

            return doc;
        }

        private static void AnalyzeStatement(IStatement stmt, DocumentationData doc, bool debug)
        {
            switch (stmt)
            {
                case FunctionDeclNode func:
                    doc.Functions.Add(new FunctionDoc
                    {
                        Name = func.Name,
                        ReturnType = func.ReturnType ?? "unknown",
                        Parameters = func.Parameters.Select(p => new ParameterDoc
                        {
                            Name = p.Name,
                            Type = p.TypeName ?? "unknown"
                        }).ToList(),
                        LineNumber = 0 // TODO: Implement line tracking
                    });
                    break;

                case VarDeclNode varDecl:
                    doc.Variables.Add(new VariableDoc
                    {
                        Name = varDecl.Identifier,
                        Type = varDecl.TypeName ?? "inferred",
                        IsExternal = varDecl.FromExternal,
                        LineNumber = 0
                    });
                    break;

                case SessionDeclNode session:
                    doc.Sessions.Add(new SessionDoc
                    {
                        Name = session.Name,
                        Members = session.Members.Select(m => new MemberDoc
                        {
                            Name = GetMemberName(m),
                            Type = GetMemberType(m),
                            Visibility = GetMemberVisibility(m)
                        }).ToList(),
                        LineNumber = 0
                    });
                    break;

                case TranceifyDeclNode tranceify:
                    doc.Tranceifies.Add(new TranceifyDoc
                    {
                        Name = tranceify.Name,
                        Members = tranceify.Members.Select(m => new MemberDoc
                        {
                            Name = GetMemberName(m),
                            Type = GetMemberType(m),
                            Visibility = "public"
                        }).ToList(),
                        LineNumber = 0
                    });
                    break;

                case MindLinkNode mindLink:
                    doc.Imports.Add(new ImportDoc
                    {
                        FileName = mindLink.FileName,
                        LineNumber = 0
                    });
                    break;

                default:
                    if (debug)
                    {
                        AppLogger.Debug($"Unhandled statement type: {stmt.GetType().Name}");
                    }
                    break;
            }
        }

        private static string GetMemberName(SessionMemberNode member)
        {
            if (member.Declaration is VarDeclNode varDecl)
                return varDecl.Identifier;
            if (member.Declaration is FunctionDeclNode funcDecl)
                return funcDecl.Name;
            return "unknown";
        }

        private static string GetMemberType(SessionMemberNode member)
        {
            if (member.Declaration is VarDeclNode varDecl)
                return varDecl.TypeName ?? "inferred";
            if (member.Declaration is FunctionDeclNode funcDecl)
                return funcDecl.ReturnType ?? "void";
            return "unknown";
        }

        private static string GetMemberVisibility(SessionMemberNode member)
        {
            return member.IsExposed ? "public" : "private";
        }

        private static string GetMemberName(VarDeclNode varDecl)
        {
            return varDecl.Identifier;
        }

        private static string GetMemberType(VarDeclNode varDecl)
        {
            return varDecl.TypeName ?? "inferred";
        }

        private static string GenerateHtmlDocumentation(DocumentationData doc)
        {
            var html = new StringBuilder();
            html.AppendLine("<!DOCTYPE html>");
            html.AppendLine("<html lang=\"de\">");
            html.AppendLine("<head>");
            html.AppendLine("    <meta charset=\"UTF-8\">");
            html.AppendLine("    <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">");
            html.AppendLine($"    <title>HypnoScript Documentation - {doc.FileName}</title>");
            html.AppendLine("    <style>");
            html.AppendLine("        body { font-family: Arial, sans-serif; margin: 40px; line-height: 1.6; }");
            html.AppendLine("        .header { background: #f0f0f0; padding: 20px; border-radius: 5px; margin-bottom: 30px; }");
            html.AppendLine("        .section { margin-bottom: 30px; }");
            html.AppendLine("        .function, .variable, .session, .tranceify { background: #f9f9f9; padding: 15px; margin: 10px 0; border-left: 4px solid #007acc; }");
            html.AppendLine("        .name { font-weight: bold; color: #007acc; }");
            html.AppendLine("        .type { color: #666; font-style: italic; }");
            html.AppendLine("        .parameter { background: #e8f4f8; padding: 5px; margin: 2px; border-radius: 3px; display: inline-block; }");
            html.AppendLine("        .stats { display: grid; grid-template-columns: repeat(auto-fit, minmax(200px, 1fr)); gap: 20px; margin: 20px 0; }");
            html.AppendLine("        .stat { background: #e8f4f8; padding: 15px; border-radius: 5px; text-align: center; }");
            html.AppendLine("        .stat-number { font-size: 2em; font-weight: bold; color: #007acc; }");
            html.AppendLine("    </style>");
            html.AppendLine("</head>");
            html.AppendLine("<body>");

            // Header
            html.AppendLine("    <div class=\"header\">");
            html.AppendLine($"        <h1>HypnoScript Documentation</h1>");
            html.AppendLine($"        <p><strong>File:</strong> {doc.FileName}</p>");
            html.AppendLine($"        <p><strong>Generated:</strong> {doc.GeneratedAt:yyyy-MM-dd HH:mm:ss}</p>");
            html.AppendLine("    </div>");

            // Statistics
            html.AppendLine("    <div class=\"stats\">");
            html.AppendLine($"        <div class=\"stat\"><div class=\"stat-number\">{doc.Functions.Count}</div><div>Functions</div></div>");
            html.AppendLine($"        <div class=\"stat\"><div class=\"stat-number\">{doc.Variables.Count}</div><div>Variables</div></div>");
            html.AppendLine($"        <div class=\"stat\"><div class=\"stat-number\">{doc.Sessions.Count}</div><div>Sessions</div></div>");
            html.AppendLine($"        <div class=\"stat\"><div class=\"stat-number\">{doc.Tranceifies.Count}</div><div>Tranceifies</div></div>");
            html.AppendLine($"        <div class=\"stat\"><div class=\"stat-number\">{doc.LinesOfCode}</div><div>Lines of Code</div></div>");
            html.AppendLine("    </div>");

            // Functions
            if (doc.Functions.Any())
            {
                html.AppendLine("    <div class=\"section\">");
                html.AppendLine("        <h2>Functions</h2>");
                foreach (var func in doc.Functions)
                {
                    html.AppendLine("        <div class=\"function\">");
                    html.AppendLine($"            <div class=\"name\">{func.Name}</div>");
                    html.AppendLine($"            <div class=\"type\">Returns: {func.ReturnType}</div>");
                    if (func.Parameters.Any())
                    {
                        html.AppendLine("            <div>Parameters:</div>");
                        foreach (var param in func.Parameters)
                        {
                            html.AppendLine($"                <span class=\"parameter\">{param.Name}: {param.Type}</span>");
                        }
                    }
                    html.AppendLine("        </div>");
                }
                html.AppendLine("    </div>");
            }

            // Variables
            if (doc.Variables.Any())
            {
                html.AppendLine("    <div class=\"section\">");
                html.AppendLine("        <h2>Variables</h2>");
                foreach (var var in doc.Variables)
                {
                    html.AppendLine("        <div class=\"variable\">");
                    html.AppendLine($"            <div class=\"name\">{var.Name}</div>");
                    html.AppendLine($"            <div class=\"type\">Type: {var.Type}</div>");
                    if (var.IsExternal)
                    {
                        html.AppendLine("            <div><em>External input</em></div>");
                    }
                    html.AppendLine("        </div>");
                }
                html.AppendLine("    </div>");
            }

            // Sessions
            if (doc.Sessions.Any())
            {
                html.AppendLine("    <div class=\"section\">");
                html.AppendLine("        <h2>Sessions</h2>");
                foreach (var session in doc.Sessions)
                {
                    html.AppendLine("        <div class=\"session\">");
                    html.AppendLine($"            <div class=\"name\">{session.Name}</div>");
                    if (session.Members.Any())
                    {
                        html.AppendLine("            <div>Members:</div>");
                        foreach (var member in session.Members)
                        {
                            html.AppendLine($"                <div class=\"parameter\">{member.Visibility} {member.Name}: {member.Type}</div>");
                        }
                    }
                    html.AppendLine("        </div>");
                }
                html.AppendLine("    </div>");
            }

            // Tranceifies
            if (doc.Tranceifies.Any())
            {
                html.AppendLine("    <div class=\"section\">");
                html.AppendLine("        <h2>Tranceifies</h2>");
                foreach (var tranceify in doc.Tranceifies)
                {
                    html.AppendLine("        <div class=\"tranceify\">");
                    html.AppendLine($"            <div class=\"name\">{tranceify.Name}</div>");
                    if (tranceify.Members.Any())
                    {
                        html.AppendLine("            <div>Members:</div>");
                        foreach (var member in tranceify.Members)
                        {
                            html.AppendLine($"                <div class=\"parameter\">{member.Name}: {member.Type}</div>");
                        }
                    }
                    html.AppendLine("        </div>");
                }
                html.AppendLine("    </div>");
            }

            // Imports
            if (doc.Imports.Any())
            {
                html.AppendLine("    <div class=\"section\">");
                html.AppendLine("        <h2>Imports</h2>");
                foreach (var import in doc.Imports)
                {
                    html.AppendLine($"        <div class=\"parameter\">{import.FileName}</div>");
                }
                html.AppendLine("    </div>");
            }

            html.AppendLine("</body>");
            html.AppendLine("</html>");

            return html.ToString();
        }

        private static string GenerateMarkdownDocumentation(DocumentationData doc)
        {
            var markdown = new StringBuilder();

            // Header
            markdown.AppendLine($"# HypnoScript Documentation - {doc.FileName}");
            markdown.AppendLine();
            markdown.AppendLine($"**Generated:** {doc.GeneratedAt:yyyy-MM-dd HH:mm:ss}");
            markdown.AppendLine($"**Lines of Code:** {doc.LinesOfCode}");
            markdown.AppendLine();

            // Statistics
            markdown.AppendLine("## Statistics");
            markdown.AppendLine();
            markdown.AppendLine($"- **Functions:** {doc.Functions.Count}");
            markdown.AppendLine($"- **Variables:** {doc.Variables.Count}");
            markdown.AppendLine($"- **Sessions:** {doc.Sessions.Count}");
            markdown.AppendLine($"- **Tranceifies:** {doc.Tranceifies.Count}");
            markdown.AppendLine($"- **Imports:** {doc.Imports.Count}");
            markdown.AppendLine();

            // Functions
            if (doc.Functions.Any())
            {
                markdown.AppendLine("## Functions");
                markdown.AppendLine();
                foreach (var func in doc.Functions)
                {
                    markdown.AppendLine($"### {func.Name}");
                    markdown.AppendLine();
                    markdown.AppendLine($"**Returns:** `{func.ReturnType}`");
                    if (func.Parameters.Any())
                    {
                        markdown.AppendLine();
                        markdown.AppendLine("**Parameters:**");
                        foreach (var param in func.Parameters)
                        {
                            markdown.AppendLine($"- `{param.Name}`: `{param.Type}`");
                        }
                    }
                    markdown.AppendLine();
                }
            }

            // Variables
            if (doc.Variables.Any())
            {
                markdown.AppendLine("## Variables");
                markdown.AppendLine();
                foreach (var var in doc.Variables)
                {
                    markdown.AppendLine($"### {var.Name}");
                    markdown.AppendLine();
                    markdown.AppendLine($"**Type:** `{var.Type}`");
                    if (var.IsExternal)
                    {
                        markdown.AppendLine("**External Input:** Yes");
                    }
                    markdown.AppendLine();
                }
            }

            // Sessions
            if (doc.Sessions.Any())
            {
                markdown.AppendLine("## Sessions");
                markdown.AppendLine();
                foreach (var session in doc.Sessions)
                {
                    markdown.AppendLine($"### {session.Name}");
                    markdown.AppendLine();
                    if (session.Members.Any())
                    {
                        markdown.AppendLine("**Members:**");
                        foreach (var member in session.Members)
                        {
                            markdown.AppendLine($"- `{member.Visibility} {member.Name}: {member.Type}`");
                        }
                    }
                    markdown.AppendLine();
                }
            }

            // Tranceifies
            if (doc.Tranceifies.Any())
            {
                markdown.AppendLine("## Tranceifies");
                markdown.AppendLine();
                foreach (var tranceify in doc.Tranceifies)
                {
                    markdown.AppendLine($"### {tranceify.Name}");
                    markdown.AppendLine();
                    if (tranceify.Members.Any())
                    {
                        markdown.AppendLine("**Members:**");
                        foreach (var member in tranceify.Members)
                        {
                            markdown.AppendLine($"- `{member.Name}: {member.Type}`");
                        }
                    }
                    markdown.AppendLine();
                }
            }

            // Imports
            if (doc.Imports.Any())
            {
                markdown.AppendLine("## Imports");
                markdown.AppendLine();
                foreach (var import in doc.Imports)
                {
                    markdown.AppendLine($"- `{import.FileName}`");
                }
                markdown.AppendLine();
            }

            return markdown.ToString();
        }

        private static string GenerateJsonDocumentation(DocumentationData doc)
        {
            return System.Text.Json.JsonSerializer.Serialize(doc, new System.Text.Json.JsonSerializerOptions
            {
                WriteIndented = true
            });
        }
    }

    // Documentation data classes
    public class DocumentationData
    {
        public string FileName { get; set; } = "";
        public string FilePath { get; set; } = "";
        public DateTime GeneratedAt { get; set; }
        public int LinesOfCode { get; set; }
        public List<FunctionDoc> Functions { get; set; } = new();
        public List<VariableDoc> Variables { get; set; } = new();
        public List<SessionDoc> Sessions { get; set; } = new();
        public List<TranceifyDoc> Tranceifies { get; set; } = new();
        public List<ImportDoc> Imports { get; set; } = new();
    }

    public class FunctionDoc
    {
        public string Name { get; set; } = "";
        public string ReturnType { get; set; } = "";
        public List<ParameterDoc> Parameters { get; set; } = new();
        public int LineNumber { get; set; }
    }

    public class ParameterDoc
    {
        public string Name { get; set; } = "";
        public string Type { get; set; } = "";
    }

    public class VariableDoc
    {
        public string Name { get; set; } = "";
        public string Type { get; set; } = "";
        public bool IsExternal { get; set; }
        public int LineNumber { get; set; }
    }

    public class SessionDoc
    {
        public string Name { get; set; } = "";
        public List<MemberDoc> Members { get; set; } = new();
        public int LineNumber { get; set; }
    }

    public class TranceifyDoc
    {
        public string Name { get; set; } = "";
        public List<MemberDoc> Members { get; set; } = new();
        public int LineNumber { get; set; }
    }

    public class MemberDoc
    {
        public string Name { get; set; } = "";
        public string Type { get; set; } = "";
        public string Visibility { get; set; } = "public";
    }

    public class ImportDoc
    {
        public string FileName { get; set; } = "";
        public int LineNumber { get; set; }
    }
}
