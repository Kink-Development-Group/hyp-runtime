using System;
using System.Net.Http;
using System.Threading.Tasks;
using System.Threading;
using System.Web;

namespace HypnoScript.Runtime.Builtins
{
    /// <summary>
    /// Network Builtins für HypnoScript (ausgelagert aus HypnoBuiltins)
    /// </summary>
    public static class NetworkBuiltins
    {
        /// <summary>Führt einen asynchronen HTTP-GET-Request aus.</summary>
        public static async Task<string> HttpGetAsync(string url, CancellationToken cancellationToken = default)
        {
            try
            {
                using (var client = new HttpClient())
                {
                    client.Timeout = TimeSpan.FromSeconds(10);
                    var response = await client.GetAsync(url, cancellationToken);
                    return await response.Content.ReadAsStringAsync();
                }
            }
            catch (Exception ex)
            {
                HypnoBuiltins.Observe($"HTTP GET error: {ex.Message}");
                return string.Empty;
            }
        }
        /// <summary>Führt einen asynchronen HTTP-POST-Request aus.</summary>
        public static async Task<string> HttpPostAsync(string url, string data, CancellationToken cancellationToken = default)
        {
            try
            {
                using (var client = new HttpClient())
                {
                    client.Timeout = TimeSpan.FromSeconds(10);
                    var content = new StringContent(data, System.Text.Encoding.UTF8, "application/json");
                    var response = await client.PostAsync(url, content, cancellationToken);
                    return await response.Content.ReadAsStringAsync();
                }
            }
            catch (Exception ex)
            {
                HypnoBuiltins.Observe($"HTTP POST error: {ex.Message}");
                return string.Empty;
            }
        }
        /// <summary>Führt einen synchronen HTTP-GET-Request aus.</summary>
        public static string HttpGet(string url) => HttpGetAsync(url).GetAwaiter().GetResult();
        /// <summary>Führt einen synchronen HTTP-POST-Request aus.</summary>
        public static string HttpPost(string url, string data) => HttpPostAsync(url, data).GetAwaiter().GetResult();
        /// <summary>Prüft, ob eine E-Mail-Adresse gültig ist.</summary>
        public static bool IsValidEmail(string email)
        {
            try
            {
                var addr = new System.Net.Mail.MailAddress(email);
                return addr.Address == email;
            }
            catch { return false; }
        }
        /// <summary>Prüft, ob eine URL gültig ist.</summary>
        public static bool IsValidUrl(string url) => Uri.TryCreate(url, UriKind.Absolute, out _);
        /// <summary>Prüft, ob eine IP-Adresse gültig ist.</summary>
        public static bool IsValidIPAddress(string str) => System.Net.IPAddress.TryParse(str, out _);
        /// <summary>Prüft, ob ein Port gültig ist (1-65535).</summary>
        public static bool IsValidPort(int port) => port >= 1 && port <= 65535;
        /// <summary>URL-Encoding.</summary>
        public static string UrlEncode(string str) => HttpUtility.UrlEncode(str);
        /// <summary>URL-Decoding.</summary>
        public static string UrlDecode(string str) => HttpUtility.UrlDecode(str);
        /// <summary>HTML-Encoding.</summary>
        public static string HtmlEncode(string str) => HttpUtility.HtmlEncode(str);
        /// <summary>HTML-Decoding.</summary>
        public static string HtmlDecode(string str) => HttpUtility.HtmlDecode(str);
        /// <summary>Extrahiert die Domain aus einer URL.</summary>
        public static string ExtractDomain(string url)
        {
            try { var uri = new Uri(url); return uri.Host; } catch { return string.Empty; }
        }
        /// <summary>Extrahiert den Pfad aus einer URL.</summary>
        public static string ExtractPath(string url)
        {
            try { var uri = new Uri(url); return uri.AbsolutePath; } catch { return string.Empty; }
        }
        /// <summary>Prüft, ob eine URL auf localhost zeigt.</summary>
        public static bool IsLocalhost(string url)
        {
            try { var uri = new Uri(url); return uri.Host == "localhost" || uri.Host == "127.0.0.1"; } catch { return false; }
        }
    }
}
