using System;
using System.Net.Http;
using System.Threading.Tasks;
using System.Threading;
using System.Web;
using System.Text.Json;

namespace HypnoScript.Runtime.Builtins
{
    /// <summary>
    /// Stellt Netzwerk- und HTTP-Funktionen für HypnoScript bereit.
    /// </summary>
    public static class NetworkBuiltins
    {
        private static readonly HttpClient _httpClient = new HttpClient();

        /// <summary>
        /// Makes an HTTP GET request
        /// </summary>
        public static async Task<string> HttpGet(string url)
        {
            try
            {
                var response = await _httpClient.GetAsync(url);
                response.EnsureSuccessStatusCode();
                return await response.Content.ReadAsStringAsync();
            }
            catch (Exception ex)
            {
                throw new InvalidOperationException($"HTTP GET failed: {ex.Message}");
            }
        }

        /// <summary>
        /// Makes an HTTP POST request
        /// </summary>
        public static async Task<string> HttpPost(string url, string content)
        {
            try
            {
                var httpContent = new StringContent(content);
                var response = await _httpClient.PostAsync(url, httpContent);
                response.EnsureSuccessStatusCode();
                return await response.Content.ReadAsStringAsync();
            }
            catch (Exception ex)
            {
                throw new InvalidOperationException($"HTTP POST failed: {ex.Message}");
            }
        }

        /// <summary>
        /// Makes an HTTP POST request with JSON content
        /// </summary>
        public static async Task<string> HttpPostJson(string url, object data)
        {
            try
            {
                var json = JsonSerializer.Serialize(data);
                var httpContent = new StringContent(json, System.Text.Encoding.UTF8, "application/json");
                var response = await _httpClient.PostAsync(url, httpContent);
                response.EnsureSuccessStatusCode();
                return await response.Content.ReadAsStringAsync();
            }
            catch (Exception ex)
            {
                throw new InvalidOperationException($"HTTP POST JSON failed: {ex.Message}");
            }
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

        /// <summary>
        /// Validates email format
        /// </summary>
        public static bool IsValidEmail(string email)
        {
            if (string.IsNullOrEmpty(email)) return false;

            try
            {
                var regex = new System.Text.RegularExpressions.Regex(@"^[^@\s]+@[^@\s]+\.[^@\s]+$");
                return regex.IsMatch(email);
            }
            catch
            {
                return false;
            }
        }
    }
}
