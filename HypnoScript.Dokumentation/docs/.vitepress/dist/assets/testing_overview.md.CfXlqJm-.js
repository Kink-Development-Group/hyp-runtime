import{_ as n,c as a,o as e,ag as i}from"./chunks/framework.Dli2S8Ej.js";const k=JSON.parse('{"title":"Test-Framework Übersicht","description":"","frontmatter":{"sidebar_position":1},"headers":[],"relativePath":"testing/overview.md","filePath":"testing/overview.md","lastUpdated":1750547232000}'),p={name:"testing/overview.md"};function l(t,s,r,h,c,u){return e(),a("div",null,[...s[0]||(s[0]=[i(`<h1 id="test-framework-ubersicht" tabindex="-1">Test-Framework Übersicht <a class="header-anchor" href="#test-framework-ubersicht" aria-label="Permalink to &quot;Test-Framework Übersicht&quot;">​</a></h1><p>Das HypnoScript Test-Framework bietet umfassende Testing-Funktionalitäten für Unit-Tests, Integration-Tests und Performance-Tests.</p><h2 id="grundlagen" tabindex="-1">Grundlagen <a class="header-anchor" href="#grundlagen" aria-label="Permalink to &quot;Grundlagen&quot;">​</a></h2><h3 id="test-struktur" tabindex="-1">Test-Struktur <a class="header-anchor" href="#test-struktur" aria-label="Permalink to &quot;Test-Struktur&quot;">​</a></h3><p>Tests in HypnoScript verwenden eine spezielle Syntax mit <code>Test</code>-Blöcken:</p><div class="language-hyp vp-adaptive-theme line-numbers-mode"><button title="Copy Code" class="copy"></button><span class="lang">hyp</span><pre class="shiki shiki-themes github-light github-dark vp-code" tabindex="0"><code><span class="line"><span>Test &quot;Mein erster Test&quot; {</span></span>
<span class="line"><span>    entrance {</span></span>
<span class="line"><span>        induce result = 2 + 2;</span></span>
<span class="line"><span>        AssertEqual(result, 4);</span></span>
<span class="line"><span>    }</span></span>
<span class="line"><span>} Relax;</span></span></code></pre><div class="line-numbers-wrapper" aria-hidden="true"><span class="line-number">1</span><br><span class="line-number">2</span><br><span class="line-number">3</span><br><span class="line-number">4</span><br><span class="line-number">5</span><br><span class="line-number">6</span><br></div></div><h3 id="test-ausfuhrung" tabindex="-1">Test-Ausführung <a class="header-anchor" href="#test-ausfuhrung" aria-label="Permalink to &quot;Test-Ausführung&quot;">​</a></h3><div class="language-bash vp-adaptive-theme line-numbers-mode"><button title="Copy Code" class="copy"></button><span class="lang">bash</span><pre class="shiki shiki-themes github-light github-dark vp-code" tabindex="0"><code><span class="line"><span style="--shiki-light:#6A737D;--shiki-dark:#6A737D;"># Alle Tests ausführen</span></span>
<span class="line"><span style="--shiki-light:#6F42C1;--shiki-dark:#B392F0;">dotnet</span><span style="--shiki-light:#032F62;--shiki-dark:#9ECBFF;"> run</span><span style="--shiki-light:#005CC5;--shiki-dark:#79B8FF;"> --project</span><span style="--shiki-light:#032F62;--shiki-dark:#9ECBFF;"> HypnoScript.CLI</span><span style="--shiki-light:#005CC5;--shiki-dark:#79B8FF;"> --</span><span style="--shiki-light:#032F62;--shiki-dark:#9ECBFF;"> test</span><span style="--shiki-light:#005CC5;--shiki-dark:#79B8FF;"> *</span><span style="--shiki-light:#032F62;--shiki-dark:#9ECBFF;">.hyp</span></span>
<span class="line"></span>
<span class="line"><span style="--shiki-light:#6A737D;--shiki-dark:#6A737D;"># Spezifische Test-Datei</span></span>
<span class="line"><span style="--shiki-light:#6F42C1;--shiki-dark:#B392F0;">dotnet</span><span style="--shiki-light:#032F62;--shiki-dark:#9ECBFF;"> run</span><span style="--shiki-light:#005CC5;--shiki-dark:#79B8FF;"> --project</span><span style="--shiki-light:#032F62;--shiki-dark:#9ECBFF;"> HypnoScript.CLI</span><span style="--shiki-light:#005CC5;--shiki-dark:#79B8FF;"> --</span><span style="--shiki-light:#032F62;--shiki-dark:#9ECBFF;"> test</span><span style="--shiki-light:#032F62;--shiki-dark:#9ECBFF;"> test_math.hyp</span></span>
<span class="line"></span>
<span class="line"><span style="--shiki-light:#6A737D;--shiki-dark:#6A737D;"># Tests mit Filter</span></span>
<span class="line"><span style="--shiki-light:#6F42C1;--shiki-dark:#B392F0;">dotnet</span><span style="--shiki-light:#032F62;--shiki-dark:#9ECBFF;"> run</span><span style="--shiki-light:#005CC5;--shiki-dark:#79B8FF;"> --project</span><span style="--shiki-light:#032F62;--shiki-dark:#9ECBFF;"> HypnoScript.CLI</span><span style="--shiki-light:#005CC5;--shiki-dark:#79B8FF;"> --</span><span style="--shiki-light:#032F62;--shiki-dark:#9ECBFF;"> test</span><span style="--shiki-light:#005CC5;--shiki-dark:#79B8FF;"> *</span><span style="--shiki-light:#032F62;--shiki-dark:#9ECBFF;">.hyp</span><span style="--shiki-light:#005CC5;--shiki-dark:#79B8FF;"> --filter</span><span style="--shiki-light:#032F62;--shiki-dark:#9ECBFF;"> &quot;math&quot;</span></span>
<span class="line"></span>
<span class="line"><span style="--shiki-light:#6A737D;--shiki-dark:#6A737D;"># Parallele Ausführung</span></span>
<span class="line"><span style="--shiki-light:#6F42C1;--shiki-dark:#B392F0;">dotnet</span><span style="--shiki-light:#032F62;--shiki-dark:#9ECBFF;"> run</span><span style="--shiki-light:#005CC5;--shiki-dark:#79B8FF;"> --project</span><span style="--shiki-light:#032F62;--shiki-dark:#9ECBFF;"> HypnoScript.CLI</span><span style="--shiki-light:#005CC5;--shiki-dark:#79B8FF;"> --</span><span style="--shiki-light:#032F62;--shiki-dark:#9ECBFF;"> test</span><span style="--shiki-light:#005CC5;--shiki-dark:#79B8FF;"> *</span><span style="--shiki-light:#032F62;--shiki-dark:#9ECBFF;">.hyp</span><span style="--shiki-light:#005CC5;--shiki-dark:#79B8FF;"> --parallel</span></span></code></pre><div class="line-numbers-wrapper" aria-hidden="true"><span class="line-number">1</span><br><span class="line-number">2</span><br><span class="line-number">3</span><br><span class="line-number">4</span><br><span class="line-number">5</span><br><span class="line-number">6</span><br><span class="line-number">7</span><br><span class="line-number">8</span><br><span class="line-number">9</span><br><span class="line-number">10</span><br><span class="line-number">11</span><br></div></div><h2 id="test-syntax" tabindex="-1">Test-Syntax <a class="header-anchor" href="#test-syntax" aria-label="Permalink to &quot;Test-Syntax&quot;">​</a></h2><h3 id="einfache-tests" tabindex="-1">Einfache Tests <a class="header-anchor" href="#einfache-tests" aria-label="Permalink to &quot;Einfache Tests&quot;">​</a></h3><div class="language-hyp vp-adaptive-theme line-numbers-mode"><button title="Copy Code" class="copy"></button><span class="lang">hyp</span><pre class="shiki shiki-themes github-light github-dark vp-code" tabindex="0"><code><span class="line"><span>Test &quot;Addition funktioniert&quot; {</span></span>
<span class="line"><span>    entrance {</span></span>
<span class="line"><span>        induce a = 5;</span></span>
<span class="line"><span>        induce b = 3;</span></span>
<span class="line"><span>        induce result = a + b;</span></span>
<span class="line"><span>        AssertEqual(result, 8);</span></span>
<span class="line"><span>    }</span></span>
<span class="line"><span>} Relax;</span></span>
<span class="line"><span></span></span>
<span class="line"><span>Test &quot;String-Verkettung&quot; {</span></span>
<span class="line"><span>    entrance {</span></span>
<span class="line"><span>        induce str1 = &quot;Hallo&quot;;</span></span>
<span class="line"><span>        induce str2 = &quot;Welt&quot;;</span></span>
<span class="line"><span>        induce result = str1 + &quot; &quot; + str2;</span></span>
<span class="line"><span>        AssertEqual(result, &quot;Hallo Welt&quot;);</span></span>
<span class="line"><span>    }</span></span>
<span class="line"><span>} Relax;</span></span></code></pre><div class="line-numbers-wrapper" aria-hidden="true"><span class="line-number">1</span><br><span class="line-number">2</span><br><span class="line-number">3</span><br><span class="line-number">4</span><br><span class="line-number">5</span><br><span class="line-number">6</span><br><span class="line-number">7</span><br><span class="line-number">8</span><br><span class="line-number">9</span><br><span class="line-number">10</span><br><span class="line-number">11</span><br><span class="line-number">12</span><br><span class="line-number">13</span><br><span class="line-number">14</span><br><span class="line-number">15</span><br><span class="line-number">16</span><br><span class="line-number">17</span><br></div></div><h3 id="test-mit-setup-und-teardown" tabindex="-1">Test mit Setup und Teardown <a class="header-anchor" href="#test-mit-setup-und-teardown" aria-label="Permalink to &quot;Test mit Setup und Teardown&quot;">​</a></h3><div class="language-hyp vp-adaptive-theme line-numbers-mode"><button title="Copy Code" class="copy"></button><span class="lang">hyp</span><pre class="shiki shiki-themes github-light github-dark vp-code" tabindex="0"><code><span class="line"><span>Test &quot;Datei-Operationen&quot; {</span></span>
<span class="line"><span>    setup {</span></span>
<span class="line"><span>        WriteFile(&quot;test.txt&quot;, &quot;Test-Daten&quot;);</span></span>
<span class="line"><span>    }</span></span>
<span class="line"><span></span></span>
<span class="line"><span>    entrance {</span></span>
<span class="line"><span>        induce content = ReadFile(&quot;test.txt&quot;);</span></span>
<span class="line"><span>        AssertEqual(content, &quot;Test-Daten&quot;);</span></span>
<span class="line"><span>    }</span></span>
<span class="line"><span></span></span>
<span class="line"><span>    teardown {</span></span>
<span class="line"><span>        if (FileExists(&quot;test.txt&quot;)) {</span></span>
<span class="line"><span>            DeleteFile(&quot;test.txt&quot;);</span></span>
<span class="line"><span>        }</span></span>
<span class="line"><span>    }</span></span>
<span class="line"><span>} Relax;</span></span></code></pre><div class="line-numbers-wrapper" aria-hidden="true"><span class="line-number">1</span><br><span class="line-number">2</span><br><span class="line-number">3</span><br><span class="line-number">4</span><br><span class="line-number">5</span><br><span class="line-number">6</span><br><span class="line-number">7</span><br><span class="line-number">8</span><br><span class="line-number">9</span><br><span class="line-number">10</span><br><span class="line-number">11</span><br><span class="line-number">12</span><br><span class="line-number">13</span><br><span class="line-number">14</span><br><span class="line-number">15</span><br><span class="line-number">16</span><br></div></div><h3 id="test-gruppen" tabindex="-1">Test-Gruppen <a class="header-anchor" href="#test-gruppen" aria-label="Permalink to &quot;Test-Gruppen&quot;">​</a></h3><div class="language-hyp vp-adaptive-theme line-numbers-mode"><button title="Copy Code" class="copy"></button><span class="lang">hyp</span><pre class="shiki shiki-themes github-light github-dark vp-code" tabindex="0"><code><span class="line"><span>TestGroup &quot;Mathematische Funktionen&quot; {</span></span>
<span class="line"><span>    Test &quot;Addition&quot; {</span></span>
<span class="line"><span>        entrance {</span></span>
<span class="line"><span>            AssertEqual(2 + 2, 4);</span></span>
<span class="line"><span>        }</span></span>
<span class="line"><span>    } Relax;</span></span>
<span class="line"><span></span></span>
<span class="line"><span>    Test &quot;Subtraktion&quot; {</span></span>
<span class="line"><span>        entrance {</span></span>
<span class="line"><span>            AssertEqual(5 - 3, 2);</span></span>
<span class="line"><span>        }</span></span>
<span class="line"><span>    } Relax;</span></span>
<span class="line"><span></span></span>
<span class="line"><span>    Test &quot;Multiplikation&quot; {</span></span>
<span class="line"><span>        entrance {</span></span>
<span class="line"><span>            AssertEqual(4 * 3, 12);</span></span>
<span class="line"><span>        }</span></span>
<span class="line"><span>    } Relax;</span></span>
<span class="line"><span>} Relax;</span></span></code></pre><div class="line-numbers-wrapper" aria-hidden="true"><span class="line-number">1</span><br><span class="line-number">2</span><br><span class="line-number">3</span><br><span class="line-number">4</span><br><span class="line-number">5</span><br><span class="line-number">6</span><br><span class="line-number">7</span><br><span class="line-number">8</span><br><span class="line-number">9</span><br><span class="line-number">10</span><br><span class="line-number">11</span><br><span class="line-number">12</span><br><span class="line-number">13</span><br><span class="line-number">14</span><br><span class="line-number">15</span><br><span class="line-number">16</span><br><span class="line-number">17</span><br><span class="line-number">18</span><br><span class="line-number">19</span><br></div></div><h2 id="assertions" tabindex="-1">Assertions <a class="header-anchor" href="#assertions" aria-label="Permalink to &quot;Assertions&quot;">​</a></h2><h3 id="grundlegende-assertions" tabindex="-1">Grundlegende Assertions <a class="header-anchor" href="#grundlegende-assertions" aria-label="Permalink to &quot;Grundlegende Assertions&quot;">​</a></h3><div class="language-hyp vp-adaptive-theme line-numbers-mode"><button title="Copy Code" class="copy"></button><span class="lang">hyp</span><pre class="shiki shiki-themes github-light github-dark vp-code" tabindex="0"><code><span class="line"><span>Test &quot;Grundlegende Assertions&quot; {</span></span>
<span class="line"><span>    entrance {</span></span>
<span class="line"><span>        // Gleichheit</span></span>
<span class="line"><span>        AssertEqual(5, 5);</span></span>
<span class="line"><span>        AssertNotEqual(5, 6);</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        // Wahrheitswerte</span></span>
<span class="line"><span>        AssertTrue(true);</span></span>
<span class="line"><span>        AssertFalse(false);</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        // Null-Checks</span></span>
<span class="line"><span>        AssertNull(null);</span></span>
<span class="line"><span>        AssertNotNull(&quot;nicht null&quot;);</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        // Leere Checks</span></span>
<span class="line"><span>        AssertEmpty(&quot;&quot;);</span></span>
<span class="line"><span>        AssertNotEmpty(&quot;nicht leer&quot;);</span></span>
<span class="line"><span>    }</span></span>
<span class="line"><span>} Relax;</span></span></code></pre><div class="line-numbers-wrapper" aria-hidden="true"><span class="line-number">1</span><br><span class="line-number">2</span><br><span class="line-number">3</span><br><span class="line-number">4</span><br><span class="line-number">5</span><br><span class="line-number">6</span><br><span class="line-number">7</span><br><span class="line-number">8</span><br><span class="line-number">9</span><br><span class="line-number">10</span><br><span class="line-number">11</span><br><span class="line-number">12</span><br><span class="line-number">13</span><br><span class="line-number">14</span><br><span class="line-number">15</span><br><span class="line-number">16</span><br><span class="line-number">17</span><br><span class="line-number">18</span><br><span class="line-number">19</span><br></div></div><h3 id="erweiterte-assertions" tabindex="-1">Erweiterte Assertions <a class="header-anchor" href="#erweiterte-assertions" aria-label="Permalink to &quot;Erweiterte Assertions&quot;">​</a></h3><div class="language-hyp vp-adaptive-theme line-numbers-mode"><button title="Copy Code" class="copy"></button><span class="lang">hyp</span><pre class="shiki shiki-themes github-light github-dark vp-code" tabindex="0"><code><span class="line"><span>Test &quot;Erweiterte Assertions&quot; {</span></span>
<span class="line"><span>    entrance {</span></span>
<span class="line"><span>        induce arr = [1, 2, 3, 4, 5];</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        // Array-Assertions</span></span>
<span class="line"><span>        AssertArrayContains(arr, 3);</span></span>
<span class="line"><span>        AssertArrayNotContains(arr, 6);</span></span>
<span class="line"><span>        AssertArrayLength(arr, 5);</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        // String-Assertions</span></span>
<span class="line"><span>        induce str = &quot;HypnoScript&quot;;</span></span>
<span class="line"><span>        AssertStringContains(str, &quot;Script&quot;);</span></span>
<span class="line"><span>        AssertStringStartsWith(str, &quot;Hypno&quot;);</span></span>
<span class="line"><span>        AssertStringEndsWith(str, &quot;Script&quot;);</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        // Numerische Assertions</span></span>
<span class="line"><span>        AssertGreaterThan(10, 5);</span></span>
<span class="line"><span>        AssertLessThan(3, 7);</span></span>
<span class="line"><span>        AssertGreaterThanOrEqual(5, 5);</span></span>
<span class="line"><span>        AssertLessThanOrEqual(5, 5);</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        // Float-Assertions (mit Toleranz)</span></span>
<span class="line"><span>        AssertFloatEqual(3.14159, 3.14, 0.01);</span></span>
<span class="line"><span>    }</span></span>
<span class="line"><span>} Relax;</span></span></code></pre><div class="line-numbers-wrapper" aria-hidden="true"><span class="line-number">1</span><br><span class="line-number">2</span><br><span class="line-number">3</span><br><span class="line-number">4</span><br><span class="line-number">5</span><br><span class="line-number">6</span><br><span class="line-number">7</span><br><span class="line-number">8</span><br><span class="line-number">9</span><br><span class="line-number">10</span><br><span class="line-number">11</span><br><span class="line-number">12</span><br><span class="line-number">13</span><br><span class="line-number">14</span><br><span class="line-number">15</span><br><span class="line-number">16</span><br><span class="line-number">17</span><br><span class="line-number">18</span><br><span class="line-number">19</span><br><span class="line-number">20</span><br><span class="line-number">21</span><br><span class="line-number">22</span><br><span class="line-number">23</span><br><span class="line-number">24</span><br><span class="line-number">25</span><br></div></div><h3 id="exception-assertions" tabindex="-1">Exception-Assertions <a class="header-anchor" href="#exception-assertions" aria-label="Permalink to &quot;Exception-Assertions&quot;">​</a></h3><div class="language-hyp vp-adaptive-theme line-numbers-mode"><button title="Copy Code" class="copy"></button><span class="lang">hyp</span><pre class="shiki shiki-themes github-light github-dark vp-code" tabindex="0"><code><span class="line"><span>Test &quot;Exception-Tests&quot; {</span></span>
<span class="line"><span>    entrance {</span></span>
<span class="line"><span>        // Erwartete Exception</span></span>
<span class="line"><span>        AssertThrows(function() {</span></span>
<span class="line"><span>            throw &quot;Test-Exception&quot;;</span></span>
<span class="line"><span>        });</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        // Keine Exception</span></span>
<span class="line"><span>        AssertDoesNotThrow(function() {</span></span>
<span class="line"><span>            induce x = 1 + 1;</span></span>
<span class="line"><span>        });</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        // Spezifische Exception</span></span>
<span class="line"><span>        AssertThrowsWithMessage(function() {</span></span>
<span class="line"><span>            throw &quot;Ungültiger Wert&quot;;</span></span>
<span class="line"><span>        }, &quot;Ungültiger Wert&quot;);</span></span>
<span class="line"><span>    }</span></span>
<span class="line"><span>} Relax;</span></span></code></pre><div class="line-numbers-wrapper" aria-hidden="true"><span class="line-number">1</span><br><span class="line-number">2</span><br><span class="line-number">3</span><br><span class="line-number">4</span><br><span class="line-number">5</span><br><span class="line-number">6</span><br><span class="line-number">7</span><br><span class="line-number">8</span><br><span class="line-number">9</span><br><span class="line-number">10</span><br><span class="line-number">11</span><br><span class="line-number">12</span><br><span class="line-number">13</span><br><span class="line-number">14</span><br><span class="line-number">15</span><br><span class="line-number">16</span><br><span class="line-number">17</span><br><span class="line-number">18</span><br></div></div><h2 id="test-fixtures" tabindex="-1">Test-Fixtures <a class="header-anchor" href="#test-fixtures" aria-label="Permalink to &quot;Test-Fixtures&quot;">​</a></h2><h3 id="globale-fixtures" tabindex="-1">Globale Fixtures <a class="header-anchor" href="#globale-fixtures" aria-label="Permalink to &quot;Globale Fixtures&quot;">​</a></h3><div class="language-hyp vp-adaptive-theme line-numbers-mode"><button title="Copy Code" class="copy"></button><span class="lang">hyp</span><pre class="shiki shiki-themes github-light github-dark vp-code" tabindex="0"><code><span class="line"><span>TestFixture &quot;Datenbank-Fixture&quot; {</span></span>
<span class="line"><span>    setup {</span></span>
<span class="line"><span>        // Datenbank-Verbindung aufbauen</span></span>
<span class="line"><span>        induce connection = CreateDatabaseConnection();</span></span>
<span class="line"><span>        SetGlobalFixture(&quot;db&quot;, connection);</span></span>
<span class="line"><span>    }</span></span>
<span class="line"><span></span></span>
<span class="line"><span>    teardown {</span></span>
<span class="line"><span>        // Datenbank-Verbindung schließen</span></span>
<span class="line"><span>        induce connection = GetGlobalFixture(&quot;db&quot;);</span></span>
<span class="line"><span>        CloseDatabaseConnection(connection);</span></span>
<span class="line"><span>    }</span></span>
<span class="line"><span>} Relax;</span></span>
<span class="line"><span></span></span>
<span class="line"><span>Test &quot;Datenbank-Test&quot; {</span></span>
<span class="line"><span>    entrance {</span></span>
<span class="line"><span>        induce db = GetGlobalFixture(&quot;db&quot;);</span></span>
<span class="line"><span>        induce result = ExecuteQuery(db, &quot;SELECT COUNT(*) FROM users&quot;);</span></span>
<span class="line"><span>        AssertGreaterThan(result, 0);</span></span>
<span class="line"><span>    }</span></span>
<span class="line"><span>} Relax;</span></span></code></pre><div class="line-numbers-wrapper" aria-hidden="true"><span class="line-number">1</span><br><span class="line-number">2</span><br><span class="line-number">3</span><br><span class="line-number">4</span><br><span class="line-number">5</span><br><span class="line-number">6</span><br><span class="line-number">7</span><br><span class="line-number">8</span><br><span class="line-number">9</span><br><span class="line-number">10</span><br><span class="line-number">11</span><br><span class="line-number">12</span><br><span class="line-number">13</span><br><span class="line-number">14</span><br><span class="line-number">15</span><br><span class="line-number">16</span><br><span class="line-number">17</span><br><span class="line-number">18</span><br><span class="line-number">19</span><br><span class="line-number">20</span><br><span class="line-number">21</span><br></div></div><h3 id="test-spezifische-fixtures" tabindex="-1">Test-spezifische Fixtures <a class="header-anchor" href="#test-spezifische-fixtures" aria-label="Permalink to &quot;Test-spezifische Fixtures&quot;">​</a></h3><div class="language-hyp vp-adaptive-theme line-numbers-mode"><button title="Copy Code" class="copy"></button><span class="lang">hyp</span><pre class="shiki shiki-themes github-light github-dark vp-code" tabindex="0"><code><span class="line"><span>Test &quot;Mit Fixture&quot; {</span></span>
<span class="line"><span>    fixture {</span></span>
<span class="line"><span>        induce testData = [1, 2, 3, 4, 5];</span></span>
<span class="line"><span>        return testData;</span></span>
<span class="line"><span>    }</span></span>
<span class="line"><span></span></span>
<span class="line"><span>    entrance {</span></span>
<span class="line"><span>        induce data = GetFixture();</span></span>
<span class="line"><span>        AssertArrayLength(data, 5);</span></span>
<span class="line"><span>        AssertArrayContains(data, 3);</span></span>
<span class="line"><span>    }</span></span>
<span class="line"><span>} Relax;</span></span></code></pre><div class="line-numbers-wrapper" aria-hidden="true"><span class="line-number">1</span><br><span class="line-number">2</span><br><span class="line-number">3</span><br><span class="line-number">4</span><br><span class="line-number">5</span><br><span class="line-number">6</span><br><span class="line-number">7</span><br><span class="line-number">8</span><br><span class="line-number">9</span><br><span class="line-number">10</span><br><span class="line-number">11</span><br><span class="line-number">12</span><br></div></div><h2 id="test-parameterisierung" tabindex="-1">Test-Parameterisierung <a class="header-anchor" href="#test-parameterisierung" aria-label="Permalink to &quot;Test-Parameterisierung&quot;">​</a></h2><h3 id="parameterisierte-tests" tabindex="-1">Parameterisierte Tests <a class="header-anchor" href="#parameterisierte-tests" aria-label="Permalink to &quot;Parameterisierte Tests&quot;">​</a></h3><div class="language-hyp vp-adaptive-theme line-numbers-mode"><button title="Copy Code" class="copy"></button><span class="lang">hyp</span><pre class="shiki shiki-themes github-light github-dark vp-code" tabindex="0"><code><span class="line"><span>Test &quot;Addition mit Parametern&quot; {</span></span>
<span class="line"><span>    parameters {</span></span>
<span class="line"><span>        [2, 3, 5],</span></span>
<span class="line"><span>        [5, 7, 12],</span></span>
<span class="line"><span>        [0, 0, 0],</span></span>
<span class="line"><span>        [-1, 1, 0]</span></span>
<span class="line"><span>    }</span></span>
<span class="line"><span></span></span>
<span class="line"><span>    entrance {</span></span>
<span class="line"><span>        induce [a, b, expected] = GetTestParameters();</span></span>
<span class="line"><span>        induce result = a + b;</span></span>
<span class="line"><span>        AssertEqual(result, expected);</span></span>
<span class="line"><span>    }</span></span>
<span class="line"><span>} Relax;</span></span></code></pre><div class="line-numbers-wrapper" aria-hidden="true"><span class="line-number">1</span><br><span class="line-number">2</span><br><span class="line-number">3</span><br><span class="line-number">4</span><br><span class="line-number">5</span><br><span class="line-number">6</span><br><span class="line-number">7</span><br><span class="line-number">8</span><br><span class="line-number">9</span><br><span class="line-number">10</span><br><span class="line-number">11</span><br><span class="line-number">12</span><br><span class="line-number">13</span><br><span class="line-number">14</span><br></div></div><h3 id="daten-getriebene-tests" tabindex="-1">Daten-getriebene Tests <a class="header-anchor" href="#daten-getriebene-tests" aria-label="Permalink to &quot;Daten-getriebene Tests&quot;">​</a></h3><div class="language-hyp vp-adaptive-theme line-numbers-mode"><button title="Copy Code" class="copy"></button><span class="lang">hyp</span><pre class="shiki shiki-themes github-light github-dark vp-code" tabindex="0"><code><span class="line"><span>Test &quot;String-Tests mit Daten&quot; {</span></span>
<span class="line"><span>    dataSource &quot;test_data.json&quot;</span></span>
<span class="line"><span></span></span>
<span class="line"><span>    entrance {</span></span>
<span class="line"><span>        induce [input, expected] = GetTestData();</span></span>
<span class="line"><span>        induce result = ToUpper(input);</span></span>
<span class="line"><span>        AssertEqual(result, expected);</span></span>
<span class="line"><span>    }</span></span>
<span class="line"><span>} Relax;</span></span></code></pre><div class="line-numbers-wrapper" aria-hidden="true"><span class="line-number">1</span><br><span class="line-number">2</span><br><span class="line-number">3</span><br><span class="line-number">4</span><br><span class="line-number">5</span><br><span class="line-number">6</span><br><span class="line-number">7</span><br><span class="line-number">8</span><br><span class="line-number">9</span><br></div></div><h2 id="performance-tests" tabindex="-1">Performance-Tests <a class="header-anchor" href="#performance-tests" aria-label="Permalink to &quot;Performance-Tests&quot;">​</a></h2><h3 id="benchmark-tests" tabindex="-1">Benchmark-Tests <a class="header-anchor" href="#benchmark-tests" aria-label="Permalink to &quot;Benchmark-Tests&quot;">​</a></h3><div class="language-hyp vp-adaptive-theme line-numbers-mode"><button title="Copy Code" class="copy"></button><span class="lang">hyp</span><pre class="shiki shiki-themes github-light github-dark vp-code" tabindex="0"><code><span class="line"><span>Benchmark &quot;Array-Sortierung&quot; {</span></span>
<span class="line"><span>    entrance {</span></span>
<span class="line"><span>        induce arr = Range(1, 1000);</span></span>
<span class="line"><span>        induce shuffled = Shuffle(arr);</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        induce startTime = Timestamp();</span></span>
<span class="line"><span>        induce sorted = Sort(shuffled);</span></span>
<span class="line"><span>        induce endTime = Timestamp();</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        induce duration = endTime - startTime;</span></span>
<span class="line"><span>        AssertLessThan(duration, 1.0); // Maximal 1 Sekunde</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        // Performance-Metriken speichern</span></span>
<span class="line"><span>        RecordMetric(&quot;sort_duration&quot;, duration);</span></span>
<span class="line"><span>        RecordMetric(&quot;array_size&quot;, ArrayLength(arr));</span></span>
<span class="line"><span>    }</span></span>
<span class="line"><span>} Relax;</span></span></code></pre><div class="line-numbers-wrapper" aria-hidden="true"><span class="line-number">1</span><br><span class="line-number">2</span><br><span class="line-number">3</span><br><span class="line-number">4</span><br><span class="line-number">5</span><br><span class="line-number">6</span><br><span class="line-number">7</span><br><span class="line-number">8</span><br><span class="line-number">9</span><br><span class="line-number">10</span><br><span class="line-number">11</span><br><span class="line-number">12</span><br><span class="line-number">13</span><br><span class="line-number">14</span><br><span class="line-number">15</span><br><span class="line-number">16</span><br><span class="line-number">17</span><br></div></div><h3 id="load-tests" tabindex="-1">Load-Tests <a class="header-anchor" href="#load-tests" aria-label="Permalink to &quot;Load-Tests&quot;">​</a></h3><div class="language-hyp vp-adaptive-theme line-numbers-mode"><button title="Copy Code" class="copy"></button><span class="lang">hyp</span><pre class="shiki shiki-themes github-light github-dark vp-code" tabindex="0"><code><span class="line"><span>LoadTest &quot;API-Performance&quot; {</span></span>
<span class="line"><span>    iterations 100</span></span>
<span class="line"><span>    concurrent 10</span></span>
<span class="line"><span></span></span>
<span class="line"><span>    entrance {</span></span>
<span class="line"><span>        induce startTime = Timestamp();</span></span>
<span class="line"><span>        induce response = HttpGet(&quot;https://api.example.com/data&quot;);</span></span>
<span class="line"><span>        induce endTime = Timestamp();</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        induce responseTime = (endTime - startTime) * 1000; // in ms</span></span>
<span class="line"><span>        AssertLessThan(responseTime, 500); // Maximal 500ms</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        RecordMetric(&quot;response_time&quot;, responseTime);</span></span>
<span class="line"><span>        RecordMetric(&quot;response_size&quot;, Length(response));</span></span>
<span class="line"><span>    }</span></span>
<span class="line"><span>} Relax;</span></span></code></pre><div class="line-numbers-wrapper" aria-hidden="true"><span class="line-number">1</span><br><span class="line-number">2</span><br><span class="line-number">3</span><br><span class="line-number">4</span><br><span class="line-number">5</span><br><span class="line-number">6</span><br><span class="line-number">7</span><br><span class="line-number">8</span><br><span class="line-number">9</span><br><span class="line-number">10</span><br><span class="line-number">11</span><br><span class="line-number">12</span><br><span class="line-number">13</span><br><span class="line-number">14</span><br><span class="line-number">15</span><br><span class="line-number">16</span><br></div></div><h2 id="test-reporting" tabindex="-1">Test-Reporting <a class="header-anchor" href="#test-reporting" aria-label="Permalink to &quot;Test-Reporting&quot;">​</a></h2><h3 id="verschiedene-report-formate" tabindex="-1">Verschiedene Report-Formate <a class="header-anchor" href="#verschiedene-report-formate" aria-label="Permalink to &quot;Verschiedene Report-Formate&quot;">​</a></h3><div class="language-bash vp-adaptive-theme line-numbers-mode"><button title="Copy Code" class="copy"></button><span class="lang">bash</span><pre class="shiki shiki-themes github-light github-dark vp-code" tabindex="0"><code><span class="line"><span style="--shiki-light:#6A737D;--shiki-dark:#6A737D;"># Text-Report (Standard)</span></span>
<span class="line"><span style="--shiki-light:#6F42C1;--shiki-dark:#B392F0;">dotnet</span><span style="--shiki-light:#032F62;--shiki-dark:#9ECBFF;"> run</span><span style="--shiki-light:#005CC5;--shiki-dark:#79B8FF;"> --project</span><span style="--shiki-light:#032F62;--shiki-dark:#9ECBFF;"> HypnoScript.CLI</span><span style="--shiki-light:#005CC5;--shiki-dark:#79B8FF;"> --</span><span style="--shiki-light:#032F62;--shiki-dark:#9ECBFF;"> test</span><span style="--shiki-light:#005CC5;--shiki-dark:#79B8FF;"> *</span><span style="--shiki-light:#032F62;--shiki-dark:#9ECBFF;">.hyp</span></span>
<span class="line"></span>
<span class="line"><span style="--shiki-light:#6A737D;--shiki-dark:#6A737D;"># JSON-Report</span></span>
<span class="line"><span style="--shiki-light:#6F42C1;--shiki-dark:#B392F0;">dotnet</span><span style="--shiki-light:#032F62;--shiki-dark:#9ECBFF;"> run</span><span style="--shiki-light:#005CC5;--shiki-dark:#79B8FF;"> --project</span><span style="--shiki-light:#032F62;--shiki-dark:#9ECBFF;"> HypnoScript.CLI</span><span style="--shiki-light:#005CC5;--shiki-dark:#79B8FF;"> --</span><span style="--shiki-light:#032F62;--shiki-dark:#9ECBFF;"> test</span><span style="--shiki-light:#005CC5;--shiki-dark:#79B8FF;"> *</span><span style="--shiki-light:#032F62;--shiki-dark:#9ECBFF;">.hyp</span><span style="--shiki-light:#005CC5;--shiki-dark:#79B8FF;"> --format</span><span style="--shiki-light:#032F62;--shiki-dark:#9ECBFF;"> json</span><span style="--shiki-light:#005CC5;--shiki-dark:#79B8FF;"> --output</span><span style="--shiki-light:#032F62;--shiki-dark:#9ECBFF;"> test-results.json</span></span>
<span class="line"></span>
<span class="line"><span style="--shiki-light:#6A737D;--shiki-dark:#6A737D;"># XML-Report (für CI/CD)</span></span>
<span class="line"><span style="--shiki-light:#6F42C1;--shiki-dark:#B392F0;">dotnet</span><span style="--shiki-light:#032F62;--shiki-dark:#9ECBFF;"> run</span><span style="--shiki-light:#005CC5;--shiki-dark:#79B8FF;"> --project</span><span style="--shiki-light:#032F62;--shiki-dark:#9ECBFF;"> HypnoScript.CLI</span><span style="--shiki-light:#005CC5;--shiki-dark:#79B8FF;"> --</span><span style="--shiki-light:#032F62;--shiki-dark:#9ECBFF;"> test</span><span style="--shiki-light:#005CC5;--shiki-dark:#79B8FF;"> *</span><span style="--shiki-light:#032F62;--shiki-dark:#9ECBFF;">.hyp</span><span style="--shiki-light:#005CC5;--shiki-dark:#79B8FF;"> --format</span><span style="--shiki-light:#032F62;--shiki-dark:#9ECBFF;"> xml</span><span style="--shiki-light:#005CC5;--shiki-dark:#79B8FF;"> --output</span><span style="--shiki-light:#032F62;--shiki-dark:#9ECBFF;"> test-results.xml</span></span>
<span class="line"></span>
<span class="line"><span style="--shiki-light:#6A737D;--shiki-dark:#6A737D;"># HTML-Report</span></span>
<span class="line"><span style="--shiki-light:#6F42C1;--shiki-dark:#B392F0;">dotnet</span><span style="--shiki-light:#032F62;--shiki-dark:#9ECBFF;"> run</span><span style="--shiki-light:#005CC5;--shiki-dark:#79B8FF;"> --project</span><span style="--shiki-light:#032F62;--shiki-dark:#9ECBFF;"> HypnoScript.CLI</span><span style="--shiki-light:#005CC5;--shiki-dark:#79B8FF;"> --</span><span style="--shiki-light:#032F62;--shiki-dark:#9ECBFF;"> test</span><span style="--shiki-light:#005CC5;--shiki-dark:#79B8FF;"> *</span><span style="--shiki-light:#032F62;--shiki-dark:#9ECBFF;">.hyp</span><span style="--shiki-light:#005CC5;--shiki-dark:#79B8FF;"> --format</span><span style="--shiki-light:#032F62;--shiki-dark:#9ECBFF;"> html</span><span style="--shiki-light:#005CC5;--shiki-dark:#79B8FF;"> --output</span><span style="--shiki-light:#032F62;--shiki-dark:#9ECBFF;"> test-report.html</span></span></code></pre><div class="line-numbers-wrapper" aria-hidden="true"><span class="line-number">1</span><br><span class="line-number">2</span><br><span class="line-number">3</span><br><span class="line-number">4</span><br><span class="line-number">5</span><br><span class="line-number">6</span><br><span class="line-number">7</span><br><span class="line-number">8</span><br><span class="line-number">9</span><br><span class="line-number">10</span><br><span class="line-number">11</span><br></div></div><h3 id="coverage-reporting" tabindex="-1">Coverage-Reporting <a class="header-anchor" href="#coverage-reporting" aria-label="Permalink to &quot;Coverage-Reporting&quot;">​</a></h3><div class="language-bash vp-adaptive-theme line-numbers-mode"><button title="Copy Code" class="copy"></button><span class="lang">bash</span><pre class="shiki shiki-themes github-light github-dark vp-code" tabindex="0"><code><span class="line"><span style="--shiki-light:#6A737D;--shiki-dark:#6A737D;"># Code-Coverage aktivieren</span></span>
<span class="line"><span style="--shiki-light:#6F42C1;--shiki-dark:#B392F0;">dotnet</span><span style="--shiki-light:#032F62;--shiki-dark:#9ECBFF;"> run</span><span style="--shiki-light:#005CC5;--shiki-dark:#79B8FF;"> --project</span><span style="--shiki-light:#032F62;--shiki-dark:#9ECBFF;"> HypnoScript.CLI</span><span style="--shiki-light:#005CC5;--shiki-dark:#79B8FF;"> --</span><span style="--shiki-light:#032F62;--shiki-dark:#9ECBFF;"> test</span><span style="--shiki-light:#005CC5;--shiki-dark:#79B8FF;"> *</span><span style="--shiki-light:#032F62;--shiki-dark:#9ECBFF;">.hyp</span><span style="--shiki-light:#005CC5;--shiki-dark:#79B8FF;"> --coverage</span></span>
<span class="line"></span>
<span class="line"><span style="--shiki-light:#6A737D;--shiki-dark:#6A737D;"># Coverage mit Schwellenwert</span></span>
<span class="line"><span style="--shiki-light:#6F42C1;--shiki-dark:#B392F0;">dotnet</span><span style="--shiki-light:#032F62;--shiki-dark:#9ECBFF;"> run</span><span style="--shiki-light:#005CC5;--shiki-dark:#79B8FF;"> --project</span><span style="--shiki-light:#032F62;--shiki-dark:#9ECBFF;"> HypnoScript.CLI</span><span style="--shiki-light:#005CC5;--shiki-dark:#79B8FF;"> --</span><span style="--shiki-light:#032F62;--shiki-dark:#9ECBFF;"> test</span><span style="--shiki-light:#005CC5;--shiki-dark:#79B8FF;"> *</span><span style="--shiki-light:#032F62;--shiki-dark:#9ECBFF;">.hyp</span><span style="--shiki-light:#005CC5;--shiki-dark:#79B8FF;"> --coverage</span><span style="--shiki-light:#005CC5;--shiki-dark:#79B8FF;"> --coverage-threshold</span><span style="--shiki-light:#005CC5;--shiki-dark:#79B8FF;"> 80</span></span>
<span class="line"></span>
<span class="line"><span style="--shiki-light:#6A737D;--shiki-dark:#6A737D;"># Coverage-Report generieren</span></span>
<span class="line"><span style="--shiki-light:#6F42C1;--shiki-dark:#B392F0;">dotnet</span><span style="--shiki-light:#032F62;--shiki-dark:#9ECBFF;"> run</span><span style="--shiki-light:#005CC5;--shiki-dark:#79B8FF;"> --project</span><span style="--shiki-light:#032F62;--shiki-dark:#9ECBFF;"> HypnoScript.CLI</span><span style="--shiki-light:#005CC5;--shiki-dark:#79B8FF;"> --</span><span style="--shiki-light:#032F62;--shiki-dark:#9ECBFF;"> test</span><span style="--shiki-light:#005CC5;--shiki-dark:#79B8FF;"> *</span><span style="--shiki-light:#032F62;--shiki-dark:#9ECBFF;">.hyp</span><span style="--shiki-light:#005CC5;--shiki-dark:#79B8FF;"> --coverage</span><span style="--shiki-light:#005CC5;--shiki-dark:#79B8FF;"> --coverage-report</span><span style="--shiki-light:#032F62;--shiki-dark:#9ECBFF;"> html</span></span></code></pre><div class="line-numbers-wrapper" aria-hidden="true"><span class="line-number">1</span><br><span class="line-number">2</span><br><span class="line-number">3</span><br><span class="line-number">4</span><br><span class="line-number">5</span><br><span class="line-number">6</span><br><span class="line-number">7</span><br><span class="line-number">8</span><br></div></div><h2 id="test-konfiguration" tabindex="-1">Test-Konfiguration <a class="header-anchor" href="#test-konfiguration" aria-label="Permalink to &quot;Test-Konfiguration&quot;">​</a></h2><h3 id="test-konfiguration-in-hypnoscript-config-json" tabindex="-1">Test-Konfiguration in hypnoscript.config.json <a class="header-anchor" href="#test-konfiguration-in-hypnoscript-config-json" aria-label="Permalink to &quot;Test-Konfiguration in hypnoscript.config.json&quot;">​</a></h3><div class="language-json vp-adaptive-theme line-numbers-mode"><button title="Copy Code" class="copy"></button><span class="lang">json</span><pre class="shiki shiki-themes github-light github-dark vp-code" tabindex="0"><code><span class="line"><span style="--shiki-light:#24292E;--shiki-dark:#E1E4E8;">{</span></span>
<span class="line"><span style="--shiki-light:#005CC5;--shiki-dark:#79B8FF;">  &quot;testFramework&quot;</span><span style="--shiki-light:#24292E;--shiki-dark:#E1E4E8;">: {</span></span>
<span class="line"><span style="--shiki-light:#005CC5;--shiki-dark:#79B8FF;">    &quot;autoRun&quot;</span><span style="--shiki-light:#24292E;--shiki-dark:#E1E4E8;">: </span><span style="--shiki-light:#005CC5;--shiki-dark:#79B8FF;">true</span><span style="--shiki-light:#24292E;--shiki-dark:#E1E4E8;">,</span></span>
<span class="line"><span style="--shiki-light:#005CC5;--shiki-dark:#79B8FF;">    &quot;reportFormat&quot;</span><span style="--shiki-light:#24292E;--shiki-dark:#E1E4E8;">: </span><span style="--shiki-light:#032F62;--shiki-dark:#9ECBFF;">&quot;detailed&quot;</span><span style="--shiki-light:#24292E;--shiki-dark:#E1E4E8;">,</span></span>
<span class="line"><span style="--shiki-light:#005CC5;--shiki-dark:#79B8FF;">    &quot;parallelExecution&quot;</span><span style="--shiki-light:#24292E;--shiki-dark:#E1E4E8;">: </span><span style="--shiki-light:#005CC5;--shiki-dark:#79B8FF;">true</span><span style="--shiki-light:#24292E;--shiki-dark:#E1E4E8;">,</span></span>
<span class="line"><span style="--shiki-light:#005CC5;--shiki-dark:#79B8FF;">    &quot;timeout&quot;</span><span style="--shiki-light:#24292E;--shiki-dark:#E1E4E8;">: </span><span style="--shiki-light:#005CC5;--shiki-dark:#79B8FF;">30000</span><span style="--shiki-light:#24292E;--shiki-dark:#E1E4E8;">,</span></span>
<span class="line"><span style="--shiki-light:#005CC5;--shiki-dark:#79B8FF;">    &quot;coverage&quot;</span><span style="--shiki-light:#24292E;--shiki-dark:#E1E4E8;">: {</span></span>
<span class="line"><span style="--shiki-light:#005CC5;--shiki-dark:#79B8FF;">      &quot;enabled&quot;</span><span style="--shiki-light:#24292E;--shiki-dark:#E1E4E8;">: </span><span style="--shiki-light:#005CC5;--shiki-dark:#79B8FF;">true</span><span style="--shiki-light:#24292E;--shiki-dark:#E1E4E8;">,</span></span>
<span class="line"><span style="--shiki-light:#005CC5;--shiki-dark:#79B8FF;">      &quot;threshold&quot;</span><span style="--shiki-light:#24292E;--shiki-dark:#E1E4E8;">: </span><span style="--shiki-light:#005CC5;--shiki-dark:#79B8FF;">80</span><span style="--shiki-light:#24292E;--shiki-dark:#E1E4E8;">,</span></span>
<span class="line"><span style="--shiki-light:#005CC5;--shiki-dark:#79B8FF;">      &quot;excludePatterns&quot;</span><span style="--shiki-light:#24292E;--shiki-dark:#E1E4E8;">: [</span><span style="--shiki-light:#032F62;--shiki-dark:#9ECBFF;">&quot;**/test/**&quot;</span><span style="--shiki-light:#24292E;--shiki-dark:#E1E4E8;">, </span><span style="--shiki-light:#032F62;--shiki-dark:#9ECBFF;">&quot;**/vendor/**&quot;</span><span style="--shiki-light:#24292E;--shiki-dark:#E1E4E8;">]</span></span>
<span class="line"><span style="--shiki-light:#24292E;--shiki-dark:#E1E4E8;">    },</span></span>
<span class="line"><span style="--shiki-light:#005CC5;--shiki-dark:#79B8FF;">    &quot;fixtures&quot;</span><span style="--shiki-light:#24292E;--shiki-dark:#E1E4E8;">: {</span></span>
<span class="line"><span style="--shiki-light:#005CC5;--shiki-dark:#79B8FF;">      &quot;autoSetup&quot;</span><span style="--shiki-light:#24292E;--shiki-dark:#E1E4E8;">: </span><span style="--shiki-light:#005CC5;--shiki-dark:#79B8FF;">true</span><span style="--shiki-light:#24292E;--shiki-dark:#E1E4E8;">,</span></span>
<span class="line"><span style="--shiki-light:#005CC5;--shiki-dark:#79B8FF;">      &quot;autoTeardown&quot;</span><span style="--shiki-light:#24292E;--shiki-dark:#E1E4E8;">: </span><span style="--shiki-light:#005CC5;--shiki-dark:#79B8FF;">true</span></span>
<span class="line"><span style="--shiki-light:#24292E;--shiki-dark:#E1E4E8;">    },</span></span>
<span class="line"><span style="--shiki-light:#005CC5;--shiki-dark:#79B8FF;">    &quot;assertions&quot;</span><span style="--shiki-light:#24292E;--shiki-dark:#E1E4E8;">: {</span></span>
<span class="line"><span style="--shiki-light:#005CC5;--shiki-dark:#79B8FF;">      &quot;strictMode&quot;</span><span style="--shiki-light:#24292E;--shiki-dark:#E1E4E8;">: </span><span style="--shiki-light:#005CC5;--shiki-dark:#79B8FF;">true</span><span style="--shiki-light:#24292E;--shiki-dark:#E1E4E8;">,</span></span>
<span class="line"><span style="--shiki-light:#005CC5;--shiki-dark:#79B8FF;">      &quot;floatTolerance&quot;</span><span style="--shiki-light:#24292E;--shiki-dark:#E1E4E8;">: </span><span style="--shiki-light:#005CC5;--shiki-dark:#79B8FF;">0.001</span></span>
<span class="line"><span style="--shiki-light:#24292E;--shiki-dark:#E1E4E8;">    }</span></span>
<span class="line"><span style="--shiki-light:#24292E;--shiki-dark:#E1E4E8;">  }</span></span>
<span class="line"><span style="--shiki-light:#24292E;--shiki-dark:#E1E4E8;">}</span></span></code></pre><div class="line-numbers-wrapper" aria-hidden="true"><span class="line-number">1</span><br><span class="line-number">2</span><br><span class="line-number">3</span><br><span class="line-number">4</span><br><span class="line-number">5</span><br><span class="line-number">6</span><br><span class="line-number">7</span><br><span class="line-number">8</span><br><span class="line-number">9</span><br><span class="line-number">10</span><br><span class="line-number">11</span><br><span class="line-number">12</span><br><span class="line-number">13</span><br><span class="line-number">14</span><br><span class="line-number">15</span><br><span class="line-number">16</span><br><span class="line-number">17</span><br><span class="line-number">18</span><br><span class="line-number">19</span><br><span class="line-number">20</span><br><span class="line-number">21</span><br></div></div><h2 id="best-practices" tabindex="-1">Best Practices <a class="header-anchor" href="#best-practices" aria-label="Permalink to &quot;Best Practices&quot;">​</a></h2><h3 id="test-organisation" tabindex="-1">Test-Organisation <a class="header-anchor" href="#test-organisation" aria-label="Permalink to &quot;Test-Organisation&quot;">​</a></h3><div class="language-hyp vp-adaptive-theme line-numbers-mode"><button title="Copy Code" class="copy"></button><span class="lang">hyp</span><pre class="shiki shiki-themes github-light github-dark vp-code" tabindex="0"><code><span class="line"><span>// test_math.hyp</span></span>
<span class="line"><span>TestGroup &quot;Mathematische Grundoperationen&quot; {</span></span>
<span class="line"><span>    Test &quot;Addition&quot; {</span></span>
<span class="line"><span>        entrance {</span></span>
<span class="line"><span>            AssertEqual(2 + 2, 4);</span></span>
<span class="line"><span>        }</span></span>
<span class="line"><span>    } Relax;</span></span>
<span class="line"><span></span></span>
<span class="line"><span>    Test &quot;Subtraktion&quot; {</span></span>
<span class="line"><span>        entrance {</span></span>
<span class="line"><span>            AssertEqual(5 - 3, 2);</span></span>
<span class="line"><span>        }</span></span>
<span class="line"><span>    } Relax;</span></span>
<span class="line"><span>} Relax;</span></span>
<span class="line"><span></span></span>
<span class="line"><span>TestGroup &quot;Erweiterte Mathematik&quot; {</span></span>
<span class="line"><span>    Test &quot;Potenzierung&quot; {</span></span>
<span class="line"><span>        entrance {</span></span>
<span class="line"><span>            AssertEqual(Pow(2, 3), 8);</span></span>
<span class="line"><span>        }</span></span>
<span class="line"><span>    } Relax;</span></span>
<span class="line"><span></span></span>
<span class="line"><span>    Test &quot;Wurzel&quot; {</span></span>
<span class="line"><span>        entrance {</span></span>
<span class="line"><span>            AssertFloatEqual(Sqrt(16), 4, 0.001);</span></span>
<span class="line"><span>        }</span></span>
<span class="line"><span>    } Relax;</span></span>
<span class="line"><span>} Relax;</span></span></code></pre><div class="line-numbers-wrapper" aria-hidden="true"><span class="line-number">1</span><br><span class="line-number">2</span><br><span class="line-number">3</span><br><span class="line-number">4</span><br><span class="line-number">5</span><br><span class="line-number">6</span><br><span class="line-number">7</span><br><span class="line-number">8</span><br><span class="line-number">9</span><br><span class="line-number">10</span><br><span class="line-number">11</span><br><span class="line-number">12</span><br><span class="line-number">13</span><br><span class="line-number">14</span><br><span class="line-number">15</span><br><span class="line-number">16</span><br><span class="line-number">17</span><br><span class="line-number">18</span><br><span class="line-number">19</span><br><span class="line-number">20</span><br><span class="line-number">21</span><br><span class="line-number">22</span><br><span class="line-number">23</span><br><span class="line-number">24</span><br><span class="line-number">25</span><br><span class="line-number">26</span><br><span class="line-number">27</span><br><span class="line-number">28</span><br></div></div><h3 id="test-naming" tabindex="-1">Test-Naming <a class="header-anchor" href="#test-naming" aria-label="Permalink to &quot;Test-Naming&quot;">​</a></h3><div class="language-hyp vp-adaptive-theme line-numbers-mode"><button title="Copy Code" class="copy"></button><span class="lang">hyp</span><pre class="shiki shiki-themes github-light github-dark vp-code" tabindex="0"><code><span class="line"><span>// Gute Test-Namen</span></span>
<span class="line"><span>Test &quot;should_return_sum_when_adding_two_numbers&quot; { ... } Relax;</span></span>
<span class="line"><span>Test &quot;should_throw_exception_when_dividing_by_zero&quot; { ... } Relax;</span></span>
<span class="line"><span>Test &quot;should_validate_email_format_correctly&quot; { ... } Relax;</span></span>
<span class="line"><span></span></span>
<span class="line"><span>// Schlechte Test-Namen</span></span>
<span class="line"><span>Test &quot;test1&quot; { ... } Relax;</span></span>
<span class="line"><span>Test &quot;math&quot; { ... } Relax;</span></span>
<span class="line"><span>Test &quot;function&quot; { ... } Relax;</span></span></code></pre><div class="line-numbers-wrapper" aria-hidden="true"><span class="line-number">1</span><br><span class="line-number">2</span><br><span class="line-number">3</span><br><span class="line-number">4</span><br><span class="line-number">5</span><br><span class="line-number">6</span><br><span class="line-number">7</span><br><span class="line-number">8</span><br><span class="line-number">9</span><br></div></div><h3 id="test-isolation" tabindex="-1">Test-Isolation <a class="header-anchor" href="#test-isolation" aria-label="Permalink to &quot;Test-Isolation&quot;">​</a></h3><div class="language-hyp vp-adaptive-theme line-numbers-mode"><button title="Copy Code" class="copy"></button><span class="lang">hyp</span><pre class="shiki shiki-themes github-light github-dark vp-code" tabindex="0"><code><span class="line"><span>Test &quot;Isolierter Test&quot; {</span></span>
<span class="line"><span>    setup {</span></span>
<span class="line"><span>        // Jeder Test bekommt seine eigenen Daten</span></span>
<span class="line"><span>        induce testFile = &quot;test_&quot; + Timestamp() + &quot;.txt&quot;;</span></span>
<span class="line"><span>        WriteFile(testFile, &quot;Test-Daten&quot;);</span></span>
<span class="line"><span>        SetTestData(&quot;file&quot;, testFile);</span></span>
<span class="line"><span>    }</span></span>
<span class="line"><span></span></span>
<span class="line"><span>    entrance {</span></span>
<span class="line"><span>        induce file = GetTestData(&quot;file&quot;);</span></span>
<span class="line"><span>        induce content = ReadFile(file);</span></span>
<span class="line"><span>        AssertEqual(content, &quot;Test-Daten&quot;);</span></span>
<span class="line"><span>    }</span></span>
<span class="line"><span></span></span>
<span class="line"><span>    teardown {</span></span>
<span class="line"><span>        // Aufräumen</span></span>
<span class="line"><span>        induce file = GetTestData(&quot;file&quot;);</span></span>
<span class="line"><span>        if (FileExists(file)) {</span></span>
<span class="line"><span>            DeleteFile(file);</span></span>
<span class="line"><span>        }</span></span>
<span class="line"><span>    }</span></span>
<span class="line"><span>} Relax;</span></span></code></pre><div class="line-numbers-wrapper" aria-hidden="true"><span class="line-number">1</span><br><span class="line-number">2</span><br><span class="line-number">3</span><br><span class="line-number">4</span><br><span class="line-number">5</span><br><span class="line-number">6</span><br><span class="line-number">7</span><br><span class="line-number">8</span><br><span class="line-number">9</span><br><span class="line-number">10</span><br><span class="line-number">11</span><br><span class="line-number">12</span><br><span class="line-number">13</span><br><span class="line-number">14</span><br><span class="line-number">15</span><br><span class="line-number">16</span><br><span class="line-number">17</span><br><span class="line-number">18</span><br><span class="line-number">19</span><br><span class="line-number">20</span><br><span class="line-number">21</span><br><span class="line-number">22</span><br></div></div><h3 id="mocking-und-stubbing" tabindex="-1">Mocking und Stubbing <a class="header-anchor" href="#mocking-und-stubbing" aria-label="Permalink to &quot;Mocking und Stubbing&quot;">​</a></h3><div class="language-hyp vp-adaptive-theme line-numbers-mode"><button title="Copy Code" class="copy"></button><span class="lang">hyp</span><pre class="shiki shiki-themes github-light github-dark vp-code" tabindex="0"><code><span class="line"><span>Test &quot;Mit Mock&quot; {</span></span>
<span class="line"><span>    entrance {</span></span>
<span class="line"><span>        // Mock-Funktion erstellen</span></span>
<span class="line"><span>        MockFunction(&quot;HttpGet&quot;, function(url) {</span></span>
<span class="line"><span>            return &#39;{&quot;status&quot;: &quot;success&quot;, &quot;data&quot;: &quot;mocked&quot;}&#39;;</span></span>
<span class="line"><span>        });</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        induce response = HttpGet(&quot;https://api.example.com&quot;);</span></span>
<span class="line"><span>        induce data = ParseJSON(response);</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        AssertEqual(data.status, &quot;success&quot;);</span></span>
<span class="line"><span>        AssertEqual(data.data, &quot;mocked&quot;);</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        // Mock entfernen</span></span>
<span class="line"><span>        UnmockFunction(&quot;HttpGet&quot;);</span></span>
<span class="line"><span>    }</span></span>
<span class="line"><span>} Relax;</span></span></code></pre><div class="line-numbers-wrapper" aria-hidden="true"><span class="line-number">1</span><br><span class="line-number">2</span><br><span class="line-number">3</span><br><span class="line-number">4</span><br><span class="line-number">5</span><br><span class="line-number">6</span><br><span class="line-number">7</span><br><span class="line-number">8</span><br><span class="line-number">9</span><br><span class="line-number">10</span><br><span class="line-number">11</span><br><span class="line-number">12</span><br><span class="line-number">13</span><br><span class="line-number">14</span><br><span class="line-number">15</span><br><span class="line-number">16</span><br><span class="line-number">17</span><br></div></div><h2 id="ci-cd-integration" tabindex="-1">CI/CD Integration <a class="header-anchor" href="#ci-cd-integration" aria-label="Permalink to &quot;CI/CD Integration&quot;">​</a></h2><h3 id="github-actions" tabindex="-1">GitHub Actions <a class="header-anchor" href="#github-actions" aria-label="Permalink to &quot;GitHub Actions&quot;">​</a></h3><div class="language-yaml vp-adaptive-theme line-numbers-mode"><button title="Copy Code" class="copy"></button><span class="lang">yaml</span><pre class="shiki shiki-themes github-light github-dark vp-code" tabindex="0"><code><span class="line"><span style="--shiki-light:#22863A;--shiki-dark:#85E89D;">name</span><span style="--shiki-light:#24292E;--shiki-dark:#E1E4E8;">: </span><span style="--shiki-light:#032F62;--shiki-dark:#9ECBFF;">HypnoScript Tests</span></span>
<span class="line"></span>
<span class="line"><span style="--shiki-light:#005CC5;--shiki-dark:#79B8FF;">on</span><span style="--shiki-light:#24292E;--shiki-dark:#E1E4E8;">: [</span><span style="--shiki-light:#032F62;--shiki-dark:#9ECBFF;">push</span><span style="--shiki-light:#24292E;--shiki-dark:#E1E4E8;">, </span><span style="--shiki-light:#032F62;--shiki-dark:#9ECBFF;">pull_request</span><span style="--shiki-light:#24292E;--shiki-dark:#E1E4E8;">]</span></span>
<span class="line"></span>
<span class="line"><span style="--shiki-light:#22863A;--shiki-dark:#85E89D;">jobs</span><span style="--shiki-light:#24292E;--shiki-dark:#E1E4E8;">:</span></span>
<span class="line"><span style="--shiki-light:#22863A;--shiki-dark:#85E89D;">  test</span><span style="--shiki-light:#24292E;--shiki-dark:#E1E4E8;">:</span></span>
<span class="line"><span style="--shiki-light:#22863A;--shiki-dark:#85E89D;">    runs-on</span><span style="--shiki-light:#24292E;--shiki-dark:#E1E4E8;">: </span><span style="--shiki-light:#032F62;--shiki-dark:#9ECBFF;">ubuntu-latest</span></span>
<span class="line"></span>
<span class="line"><span style="--shiki-light:#22863A;--shiki-dark:#85E89D;">    steps</span><span style="--shiki-light:#24292E;--shiki-dark:#E1E4E8;">:</span></span>
<span class="line"><span style="--shiki-light:#24292E;--shiki-dark:#E1E4E8;">      - </span><span style="--shiki-light:#22863A;--shiki-dark:#85E89D;">uses</span><span style="--shiki-light:#24292E;--shiki-dark:#E1E4E8;">: </span><span style="--shiki-light:#032F62;--shiki-dark:#9ECBFF;">actions/checkout@v3</span></span>
<span class="line"></span>
<span class="line"><span style="--shiki-light:#24292E;--shiki-dark:#E1E4E8;">      - </span><span style="--shiki-light:#22863A;--shiki-dark:#85E89D;">name</span><span style="--shiki-light:#24292E;--shiki-dark:#E1E4E8;">: </span><span style="--shiki-light:#032F62;--shiki-dark:#9ECBFF;">Setup .NET</span></span>
<span class="line"><span style="--shiki-light:#22863A;--shiki-dark:#85E89D;">        uses</span><span style="--shiki-light:#24292E;--shiki-dark:#E1E4E8;">: </span><span style="--shiki-light:#032F62;--shiki-dark:#9ECBFF;">actions/setup-dotnet@v3</span></span>
<span class="line"><span style="--shiki-light:#22863A;--shiki-dark:#85E89D;">        with</span><span style="--shiki-light:#24292E;--shiki-dark:#E1E4E8;">:</span></span>
<span class="line"><span style="--shiki-light:#22863A;--shiki-dark:#85E89D;">          dotnet-version</span><span style="--shiki-light:#24292E;--shiki-dark:#E1E4E8;">: </span><span style="--shiki-light:#032F62;--shiki-dark:#9ECBFF;">&#39;8.0.x&#39;</span></span>
<span class="line"></span>
<span class="line"><span style="--shiki-light:#24292E;--shiki-dark:#E1E4E8;">      - </span><span style="--shiki-light:#22863A;--shiki-dark:#85E89D;">name</span><span style="--shiki-light:#24292E;--shiki-dark:#E1E4E8;">: </span><span style="--shiki-light:#032F62;--shiki-dark:#9ECBFF;">Run tests</span></span>
<span class="line"><span style="--shiki-light:#22863A;--shiki-dark:#85E89D;">        run</span><span style="--shiki-light:#24292E;--shiki-dark:#E1E4E8;">: </span><span style="--shiki-light:#032F62;--shiki-dark:#9ECBFF;">dotnet run --project HypnoScript.CLI -- test *.hyp --format json --output test-results.json</span></span>
<span class="line"></span>
<span class="line"><span style="--shiki-light:#24292E;--shiki-dark:#E1E4E8;">      - </span><span style="--shiki-light:#22863A;--shiki-dark:#85E89D;">name</span><span style="--shiki-light:#24292E;--shiki-dark:#E1E4E8;">: </span><span style="--shiki-light:#032F62;--shiki-dark:#9ECBFF;">Upload test results</span></span>
<span class="line"><span style="--shiki-light:#22863A;--shiki-dark:#85E89D;">        uses</span><span style="--shiki-light:#24292E;--shiki-dark:#E1E4E8;">: </span><span style="--shiki-light:#032F62;--shiki-dark:#9ECBFF;">actions/upload-artifact@v3</span></span>
<span class="line"><span style="--shiki-light:#22863A;--shiki-dark:#85E89D;">        with</span><span style="--shiki-light:#24292E;--shiki-dark:#E1E4E8;">:</span></span>
<span class="line"><span style="--shiki-light:#22863A;--shiki-dark:#85E89D;">          name</span><span style="--shiki-light:#24292E;--shiki-dark:#E1E4E8;">: </span><span style="--shiki-light:#032F62;--shiki-dark:#9ECBFF;">test-results</span></span>
<span class="line"><span style="--shiki-light:#22863A;--shiki-dark:#85E89D;">          path</span><span style="--shiki-light:#24292E;--shiki-dark:#E1E4E8;">: </span><span style="--shiki-light:#032F62;--shiki-dark:#9ECBFF;">test-results.json</span></span>
<span class="line"></span>
<span class="line"><span style="--shiki-light:#24292E;--shiki-dark:#E1E4E8;">      - </span><span style="--shiki-light:#22863A;--shiki-dark:#85E89D;">name</span><span style="--shiki-light:#24292E;--shiki-dark:#E1E4E8;">: </span><span style="--shiki-light:#032F62;--shiki-dark:#9ECBFF;">Check coverage</span></span>
<span class="line"><span style="--shiki-light:#22863A;--shiki-dark:#85E89D;">        run</span><span style="--shiki-light:#24292E;--shiki-dark:#E1E4E8;">: </span><span style="--shiki-light:#032F62;--shiki-dark:#9ECBFF;">dotnet run --project HypnoScript.CLI -- test *.hyp --coverage --coverage-threshold 80</span></span></code></pre><div class="line-numbers-wrapper" aria-hidden="true"><span class="line-number">1</span><br><span class="line-number">2</span><br><span class="line-number">3</span><br><span class="line-number">4</span><br><span class="line-number">5</span><br><span class="line-number">6</span><br><span class="line-number">7</span><br><span class="line-number">8</span><br><span class="line-number">9</span><br><span class="line-number">10</span><br><span class="line-number">11</span><br><span class="line-number">12</span><br><span class="line-number">13</span><br><span class="line-number">14</span><br><span class="line-number">15</span><br><span class="line-number">16</span><br><span class="line-number">17</span><br><span class="line-number">18</span><br><span class="line-number">19</span><br><span class="line-number">20</span><br><span class="line-number">21</span><br><span class="line-number">22</span><br><span class="line-number">23</span><br><span class="line-number">24</span><br><span class="line-number">25</span><br><span class="line-number">26</span><br><span class="line-number">27</span><br></div></div><h3 id="jenkins-pipeline" tabindex="-1">Jenkins Pipeline <a class="header-anchor" href="#jenkins-pipeline" aria-label="Permalink to &quot;Jenkins Pipeline&quot;">​</a></h3><div class="language-groovy vp-adaptive-theme line-numbers-mode"><button title="Copy Code" class="copy"></button><span class="lang">groovy</span><pre class="shiki shiki-themes github-light github-dark vp-code" tabindex="0"><code><span class="line"><span style="--shiki-light:#24292E;--shiki-dark:#E1E4E8;">pipeline {</span></span>
<span class="line"><span style="--shiki-light:#24292E;--shiki-dark:#E1E4E8;">    agent any</span></span>
<span class="line"></span>
<span class="line"><span style="--shiki-light:#24292E;--shiki-dark:#E1E4E8;">    stages {</span></span>
<span class="line"><span style="--shiki-light:#24292E;--shiki-dark:#E1E4E8;">        stage(</span><span style="--shiki-light:#032F62;--shiki-dark:#9ECBFF;">&#39;Test&#39;</span><span style="--shiki-light:#24292E;--shiki-dark:#E1E4E8;">) {</span></span>
<span class="line"><span style="--shiki-light:#24292E;--shiki-dark:#E1E4E8;">            steps {</span></span>
<span class="line"><span style="--shiki-light:#24292E;--shiki-dark:#E1E4E8;">                sh </span><span style="--shiki-light:#032F62;--shiki-dark:#9ECBFF;">&#39;dotnet run --project HypnoScript.CLI -- test *.hyp --format xml --output test-results.xml&#39;</span></span>
<span class="line"><span style="--shiki-light:#24292E;--shiki-dark:#E1E4E8;">            }</span></span>
<span class="line"><span style="--shiki-light:#24292E;--shiki-dark:#E1E4E8;">            post {</span></span>
<span class="line"><span style="--shiki-light:#24292E;--shiki-dark:#E1E4E8;">                always {</span></span>
<span class="line"><span style="--shiki-light:#24292E;--shiki-dark:#E1E4E8;">                    publishTestResults </span><span style="--shiki-light:#005CC5;--shiki-dark:#79B8FF;">testResultsPattern</span><span style="--shiki-light:#24292E;--shiki-dark:#E1E4E8;">: </span><span style="--shiki-light:#032F62;--shiki-dark:#9ECBFF;">&#39;test-results.xml&#39;</span></span>
<span class="line"><span style="--shiki-light:#24292E;--shiki-dark:#E1E4E8;">                }</span></span>
<span class="line"><span style="--shiki-light:#24292E;--shiki-dark:#E1E4E8;">            }</span></span>
<span class="line"><span style="--shiki-light:#24292E;--shiki-dark:#E1E4E8;">        }</span></span>
<span class="line"></span>
<span class="line"><span style="--shiki-light:#24292E;--shiki-dark:#E1E4E8;">        stage(</span><span style="--shiki-light:#032F62;--shiki-dark:#9ECBFF;">&#39;Coverage&#39;</span><span style="--shiki-light:#24292E;--shiki-dark:#E1E4E8;">) {</span></span>
<span class="line"><span style="--shiki-light:#24292E;--shiki-dark:#E1E4E8;">            steps {</span></span>
<span class="line"><span style="--shiki-light:#24292E;--shiki-dark:#E1E4E8;">                sh </span><span style="--shiki-light:#032F62;--shiki-dark:#9ECBFF;">&#39;dotnet run --project HypnoScript.CLI -- test *.hyp --coverage --coverage-report html&#39;</span></span>
<span class="line"><span style="--shiki-light:#24292E;--shiki-dark:#E1E4E8;">            }</span></span>
<span class="line"><span style="--shiki-light:#24292E;--shiki-dark:#E1E4E8;">            post {</span></span>
<span class="line"><span style="--shiki-light:#24292E;--shiki-dark:#E1E4E8;">                always {</span></span>
<span class="line"><span style="--shiki-light:#24292E;--shiki-dark:#E1E4E8;">                    publishHTML([</span></span>
<span class="line"><span style="--shiki-light:#005CC5;--shiki-dark:#79B8FF;">                        allowMissing</span><span style="--shiki-light:#24292E;--shiki-dark:#E1E4E8;">: </span><span style="--shiki-light:#005CC5;--shiki-dark:#79B8FF;">false</span><span style="--shiki-light:#24292E;--shiki-dark:#E1E4E8;">,</span></span>
<span class="line"><span style="--shiki-light:#005CC5;--shiki-dark:#79B8FF;">                        alwaysLinkToLastBuild</span><span style="--shiki-light:#24292E;--shiki-dark:#E1E4E8;">: </span><span style="--shiki-light:#005CC5;--shiki-dark:#79B8FF;">true</span><span style="--shiki-light:#24292E;--shiki-dark:#E1E4E8;">,</span></span>
<span class="line"><span style="--shiki-light:#005CC5;--shiki-dark:#79B8FF;">                        keepAll</span><span style="--shiki-light:#24292E;--shiki-dark:#E1E4E8;">: </span><span style="--shiki-light:#005CC5;--shiki-dark:#79B8FF;">true</span><span style="--shiki-light:#24292E;--shiki-dark:#E1E4E8;">,</span></span>
<span class="line"><span style="--shiki-light:#005CC5;--shiki-dark:#79B8FF;">                        reportDir</span><span style="--shiki-light:#24292E;--shiki-dark:#E1E4E8;">: </span><span style="--shiki-light:#032F62;--shiki-dark:#9ECBFF;">&#39;coverage&#39;</span><span style="--shiki-light:#24292E;--shiki-dark:#E1E4E8;">,</span></span>
<span class="line"><span style="--shiki-light:#005CC5;--shiki-dark:#79B8FF;">                        reportFiles</span><span style="--shiki-light:#24292E;--shiki-dark:#E1E4E8;">: </span><span style="--shiki-light:#032F62;--shiki-dark:#9ECBFF;">&#39;index.html&#39;</span><span style="--shiki-light:#24292E;--shiki-dark:#E1E4E8;">,</span></span>
<span class="line"><span style="--shiki-light:#005CC5;--shiki-dark:#79B8FF;">                        reportName</span><span style="--shiki-light:#24292E;--shiki-dark:#E1E4E8;">: </span><span style="--shiki-light:#032F62;--shiki-dark:#9ECBFF;">&#39;Coverage Report&#39;</span></span>
<span class="line"><span style="--shiki-light:#24292E;--shiki-dark:#E1E4E8;">                    ])</span></span>
<span class="line"><span style="--shiki-light:#24292E;--shiki-dark:#E1E4E8;">                }</span></span>
<span class="line"><span style="--shiki-light:#24292E;--shiki-dark:#E1E4E8;">            }</span></span>
<span class="line"><span style="--shiki-light:#24292E;--shiki-dark:#E1E4E8;">        }</span></span>
<span class="line"><span style="--shiki-light:#24292E;--shiki-dark:#E1E4E8;">    }</span></span>
<span class="line"><span style="--shiki-light:#24292E;--shiki-dark:#E1E4E8;">}</span></span></code></pre><div class="line-numbers-wrapper" aria-hidden="true"><span class="line-number">1</span><br><span class="line-number">2</span><br><span class="line-number">3</span><br><span class="line-number">4</span><br><span class="line-number">5</span><br><span class="line-number">6</span><br><span class="line-number">7</span><br><span class="line-number">8</span><br><span class="line-number">9</span><br><span class="line-number">10</span><br><span class="line-number">11</span><br><span class="line-number">12</span><br><span class="line-number">13</span><br><span class="line-number">14</span><br><span class="line-number">15</span><br><span class="line-number">16</span><br><span class="line-number">17</span><br><span class="line-number">18</span><br><span class="line-number">19</span><br><span class="line-number">20</span><br><span class="line-number">21</span><br><span class="line-number">22</span><br><span class="line-number">23</span><br><span class="line-number">24</span><br><span class="line-number">25</span><br><span class="line-number">26</span><br><span class="line-number">27</span><br><span class="line-number">28</span><br><span class="line-number">29</span><br><span class="line-number">30</span><br><span class="line-number">31</span><br><span class="line-number">32</span><br><span class="line-number">33</span><br><span class="line-number">34</span><br></div></div><h2 id="nachste-schritte" tabindex="-1">Nächste Schritte <a class="header-anchor" href="#nachste-schritte" aria-label="Permalink to &quot;Nächste Schritte&quot;">​</a></h2><ul><li><a href="./assertions.html">Test-Assertions</a> - Detaillierte Assertion-Referenz</li><li><a href="./fixtures.html">Test-Fixtures</a> - Erweiterte Fixture-Funktionen</li><li><a href="./performance.html">Performance-Testing</a> - Performance-Test-Guide</li><li><a href="./reporting.html">Test-Reporting</a> - Report-Konfiguration</li></ul><hr><p><strong>Test-Framework gemeistert? Dann lerne <a href="./assertions.html">Test-Assertions</a> kennen!</strong> ✅</p>`,63)])])}const o=n(p,[["render",l]]);export{k as __pageData,o as default};
