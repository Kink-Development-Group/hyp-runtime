import{_ as s,c as a,o as e,ag as p}from"./chunks/framework.Dli2S8Ej.js";const d=JSON.parse('{"title":"Beispiele: Utility-Funktionen","description":"","frontmatter":{"sidebar_position":1},"headers":[],"relativePath":"examples/utility-examples.md","filePath":"examples/utility-examples.md","lastUpdated":1750547232000}'),l={name:"examples/utility-examples.md"};function i(r,n,u,t,c,b){return e(),a("div",null,[...n[0]||(n[0]=[p(`<h1 id="beispiele-utility-funktionen" tabindex="-1">Beispiele: Utility-Funktionen <a class="header-anchor" href="#beispiele-utility-funktionen" aria-label="Permalink to &quot;Beispiele: Utility-Funktionen&quot;">​</a></h1><p>Diese Seite zeigt praxisnahe Beispiele für den Einsatz von Utility-Funktionen in HypnoScript. Die Beispiele sind kommentiert und können direkt übernommen oder angepasst werden.</p><h2 id="dynamische-typumwandlung-und-validierung" tabindex="-1">Dynamische Typumwandlung und Validierung <a class="header-anchor" href="#dynamische-typumwandlung-und-validierung" aria-label="Permalink to &quot;Dynamische Typumwandlung und Validierung&quot;">​</a></h2><div class="language-hyp vp-adaptive-theme line-numbers-mode"><button title="Copy Code" class="copy"></button><span class="lang">hyp</span><pre class="shiki shiki-themes github-light github-dark vp-code" tabindex="0"><code><span class="line"><span>Focus {</span></span>
<span class="line"><span>    entrance {</span></span>
<span class="line"><span>        induce input = &quot;42&quot;;</span></span>
<span class="line"><span>        induce n = ToNumber(input);</span></span>
<span class="line"><span>        if (IsNumber(n)) {</span></span>
<span class="line"><span>            observe &quot;Eingegebene Zahl: &quot; + n;</span></span>
<span class="line"><span>        } else {</span></span>
<span class="line"><span>            observe &quot;Ungültige Eingabe!&quot;;</span></span>
<span class="line"><span>        }</span></span>
<span class="line"><span>    }</span></span>
<span class="line"><span>} Relax;</span></span></code></pre><div class="line-numbers-wrapper" aria-hidden="true"><span class="line-number">1</span><br><span class="line-number">2</span><br><span class="line-number">3</span><br><span class="line-number">4</span><br><span class="line-number">5</span><br><span class="line-number">6</span><br><span class="line-number">7</span><br><span class="line-number">8</span><br><span class="line-number">9</span><br><span class="line-number">10</span><br><span class="line-number">11</span><br></div></div><h2 id="zufallige-auswahl-und-mischen" tabindex="-1">Zufällige Auswahl und Mischen <a class="header-anchor" href="#zufallige-auswahl-und-mischen" aria-label="Permalink to &quot;Zufällige Auswahl und Mischen&quot;">​</a></h2><div class="language-hyp vp-adaptive-theme line-numbers-mode"><button title="Copy Code" class="copy"></button><span class="lang">hyp</span><pre class="shiki shiki-themes github-light github-dark vp-code" tabindex="0"><code><span class="line"><span>Focus {</span></span>
<span class="line"><span>    entrance {</span></span>
<span class="line"><span>        induce namen = [&quot;Anna&quot;, &quot;Ben&quot;, &quot;Carla&quot;, &quot;Dieter&quot;];</span></span>
<span class="line"><span>        induce gewinner = Sample(namen, 1);</span></span>
<span class="line"><span>        observe &quot;Gewinner: &quot; + gewinner;</span></span>
<span class="line"><span>        induce gemischt = Shuffle(namen);</span></span>
<span class="line"><span>        observe &quot;Zufällige Reihenfolge: &quot; + gemischt;</span></span>
<span class="line"><span>    }</span></span>
<span class="line"><span>} Relax;</span></span></code></pre><div class="line-numbers-wrapper" aria-hidden="true"><span class="line-number">1</span><br><span class="line-number">2</span><br><span class="line-number">3</span><br><span class="line-number">4</span><br><span class="line-number">5</span><br><span class="line-number">6</span><br><span class="line-number">7</span><br><span class="line-number">8</span><br><span class="line-number">9</span><br></div></div><h2 id="zeitmessung-und-sleep" tabindex="-1">Zeitmessung und Sleep <a class="header-anchor" href="#zeitmessung-und-sleep" aria-label="Permalink to &quot;Zeitmessung und Sleep&quot;">​</a></h2><div class="language-hyp vp-adaptive-theme line-numbers-mode"><button title="Copy Code" class="copy"></button><span class="lang">hyp</span><pre class="shiki shiki-themes github-light github-dark vp-code" tabindex="0"><code><span class="line"><span>Focus {</span></span>
<span class="line"><span>    entrance {</span></span>
<span class="line"><span>        induce start = Timestamp();</span></span>
<span class="line"><span>        Sleep(500); // 0,5 Sekunden warten</span></span>
<span class="line"><span>        induce ende = Timestamp();</span></span>
<span class="line"><span>        observe &quot;Dauer: &quot; + (ende - start) + &quot; Sekunden&quot;;</span></span>
<span class="line"><span>    }</span></span>
<span class="line"><span>} Relax;</span></span></code></pre><div class="line-numbers-wrapper" aria-hidden="true"><span class="line-number">1</span><br><span class="line-number">2</span><br><span class="line-number">3</span><br><span class="line-number">4</span><br><span class="line-number">5</span><br><span class="line-number">6</span><br><span class="line-number">7</span><br><span class="line-number">8</span><br></div></div><h2 id="array-transformationen" tabindex="-1">Array-Transformationen <a class="header-anchor" href="#array-transformationen" aria-label="Permalink to &quot;Array-Transformationen&quot;">​</a></h2><div class="language-hyp vp-adaptive-theme line-numbers-mode"><button title="Copy Code" class="copy"></button><span class="lang">hyp</span><pre class="shiki shiki-themes github-light github-dark vp-code" tabindex="0"><code><span class="line"><span>Focus {</span></span>
<span class="line"><span>    entrance {</span></span>
<span class="line"><span>        induce zahlen = [1,2,3,4,5,2,3,4];</span></span>
<span class="line"><span>        induce unique = Unique(zahlen);</span></span>
<span class="line"><span>        observe &quot;Ohne Duplikate: &quot; + unique;</span></span>
<span class="line"><span>        induce sortiert = Sort(unique);</span></span>
<span class="line"><span>        observe &quot;Sortiert: &quot; + sortiert;</span></span>
<span class="line"><span>        induce gepaart = Zip(unique, [&quot;a&quot;,&quot;b&quot;,&quot;c&quot;,&quot;d&quot;,&quot;e&quot;]);</span></span>
<span class="line"><span>        observe &quot;Gepaart: &quot; + gepaart;</span></span>
<span class="line"><span>    }</span></span>
<span class="line"><span>} Relax;</span></span></code></pre><div class="line-numbers-wrapper" aria-hidden="true"><span class="line-number">1</span><br><span class="line-number">2</span><br><span class="line-number">3</span><br><span class="line-number">4</span><br><span class="line-number">5</span><br><span class="line-number">6</span><br><span class="line-number">7</span><br><span class="line-number">8</span><br><span class="line-number">9</span><br><span class="line-number">10</span><br><span class="line-number">11</span><br></div></div><h2 id="fehlerbehandlung-mit-try" tabindex="-1">Fehlerbehandlung mit Try <a class="header-anchor" href="#fehlerbehandlung-mit-try" aria-label="Permalink to &quot;Fehlerbehandlung mit Try&quot;">​</a></h2><div class="language-hyp vp-adaptive-theme line-numbers-mode"><button title="Copy Code" class="copy"></button><span class="lang">hyp</span><pre class="shiki shiki-themes github-light github-dark vp-code" tabindex="0"><code><span class="line"><span>Focus {</span></span>
<span class="line"><span>    Trance safeDivide(a, b) {</span></span>
<span class="line"><span>        return Try(a / b, &quot;Fehler: Division durch Null&quot;);</span></span>
<span class="line"><span>    }</span></span>
<span class="line"><span>    entrance {</span></span>
<span class="line"><span>        observe safeDivide(10, 2); // 5</span></span>
<span class="line"><span>        observe safeDivide(10, 0); // &quot;Fehler: Division durch Null&quot;</span></span>
<span class="line"><span>    }</span></span>
<span class="line"><span>} Relax;</span></span></code></pre><div class="line-numbers-wrapper" aria-hidden="true"><span class="line-number">1</span><br><span class="line-number">2</span><br><span class="line-number">3</span><br><span class="line-number">4</span><br><span class="line-number">5</span><br><span class="line-number">6</span><br><span class="line-number">7</span><br><span class="line-number">8</span><br><span class="line-number">9</span><br></div></div><h2 id="json-parsing-und-erzeugung" tabindex="-1">JSON-Parsing und -Erzeugung <a class="header-anchor" href="#json-parsing-und-erzeugung" aria-label="Permalink to &quot;JSON-Parsing und -Erzeugung&quot;">​</a></h2><div class="language-hyp vp-adaptive-theme line-numbers-mode"><button title="Copy Code" class="copy"></button><span class="lang">hyp</span><pre class="shiki shiki-themes github-light github-dark vp-code" tabindex="0"><code><span class="line"><span>Focus {</span></span>
<span class="line"><span>    entrance {</span></span>
<span class="line"><span>        induce jsonString = &#39;{&quot;name&quot;: &quot;Max&quot;, &quot;age&quot;: 30}&#39;;</span></span>
<span class="line"><span>        induce obj = ParseJSON(jsonString);</span></span>
<span class="line"><span>        observe &quot;Name: &quot; + obj.name;</span></span>
<span class="line"><span>        observe &quot;Alter: &quot; + obj.age;</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        induce arr = [1,2,3];</span></span>
<span class="line"><span>        induce jsonArr = StringifyJSON(arr);</span></span>
<span class="line"><span>        observe &quot;JSON-Array: &quot; + jsonArr;</span></span>
<span class="line"><span>    }</span></span>
<span class="line"><span>} Relax;</span></span></code></pre><div class="line-numbers-wrapper" aria-hidden="true"><span class="line-number">1</span><br><span class="line-number">2</span><br><span class="line-number">3</span><br><span class="line-number">4</span><br><span class="line-number">5</span><br><span class="line-number">6</span><br><span class="line-number">7</span><br><span class="line-number">8</span><br><span class="line-number">9</span><br><span class="line-number">10</span><br><span class="line-number">11</span><br><span class="line-number">12</span><br></div></div><h2 id="range-und-repeat" tabindex="-1">Range und Repeat <a class="header-anchor" href="#range-und-repeat" aria-label="Permalink to &quot;Range und Repeat&quot;">​</a></h2><div class="language-hyp vp-adaptive-theme line-numbers-mode"><button title="Copy Code" class="copy"></button><span class="lang">hyp</span><pre class="shiki shiki-themes github-light github-dark vp-code" tabindex="0"><code><span class="line"><span>Focus {</span></span>
<span class="line"><span>    entrance {</span></span>
<span class="line"><span>        induce r = Range(1, 5);</span></span>
<span class="line"><span>        observe &quot;Range: &quot; + r; // [1,2,3,4,5]</span></span>
<span class="line"><span>        induce rep = Repeat(&quot;A&quot;, 3);</span></span>
<span class="line"><span>        observe &quot;Repeat: &quot; + rep; // [&quot;A&quot;,&quot;A&quot;,&quot;A&quot;]</span></span>
<span class="line"><span>    }</span></span>
<span class="line"><span>} Relax;</span></span></code></pre><div class="line-numbers-wrapper" aria-hidden="true"><span class="line-number">1</span><br><span class="line-number">2</span><br><span class="line-number">3</span><br><span class="line-number">4</span><br><span class="line-number">5</span><br><span class="line-number">6</span><br><span class="line-number">7</span><br><span class="line-number">8</span><br></div></div><h2 id="kombinierte-utility-workflows" tabindex="-1">Kombinierte Utility-Workflows <a class="header-anchor" href="#kombinierte-utility-workflows" aria-label="Permalink to &quot;Kombinierte Utility-Workflows&quot;">​</a></h2><div class="language-hyp vp-adaptive-theme line-numbers-mode"><button title="Copy Code" class="copy"></button><span class="lang">hyp</span><pre class="shiki shiki-themes github-light github-dark vp-code" tabindex="0"><code><span class="line"><span>Focus {</span></span>
<span class="line"><span>    entrance {</span></span>
<span class="line"><span>        // Eingabe validieren und verarbeiten</span></span>
<span class="line"><span>        induce input = &quot;15&quot;;</span></span>
<span class="line"><span>        induce n = ToNumber(input);</span></span>
<span class="line"><span>        if (IsNumber(n) &amp;&amp; n &gt; 10) {</span></span>
<span class="line"><span>            observe &quot;Eingabe ist eine Zahl &gt; 10: &quot; + n;</span></span>
<span class="line"><span>        } else {</span></span>
<span class="line"><span>            observe &quot;Ungültige oder zu kleine Zahl!&quot;;</span></span>
<span class="line"><span>        }</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        // Zufällige Auswahl aus Range</span></span>
<span class="line"><span>        induce zahlen = Range(1, 100);</span></span>
<span class="line"><span>        induce zufall = Sample(zahlen, 5);</span></span>
<span class="line"><span>        observe &quot;5 zufällige Zahlen: &quot; + zufall;</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        // Array-Transformationen kombinieren</span></span>
<span class="line"><span>        induce arr = [1,2,2,3,4,4,5];</span></span>
<span class="line"><span>        induce clean = Sort(Unique(arr));</span></span>
<span class="line"><span>        observe &quot;Sortiert &amp; eindeutig: &quot; + clean;</span></span>
<span class="line"><span>    }</span></span>
<span class="line"><span>} Relax;</span></span></code></pre><div class="line-numbers-wrapper" aria-hidden="true"><span class="line-number">1</span><br><span class="line-number">2</span><br><span class="line-number">3</span><br><span class="line-number">4</span><br><span class="line-number">5</span><br><span class="line-number">6</span><br><span class="line-number">7</span><br><span class="line-number">8</span><br><span class="line-number">9</span><br><span class="line-number">10</span><br><span class="line-number">11</span><br><span class="line-number">12</span><br><span class="line-number">13</span><br><span class="line-number">14</span><br><span class="line-number">15</span><br><span class="line-number">16</span><br><span class="line-number">17</span><br><span class="line-number">18</span><br><span class="line-number">19</span><br><span class="line-number">20</span><br><span class="line-number">21</span><br><span class="line-number">22</span><br></div></div><hr><p><strong>Siehe auch:</strong></p><ul><li><a href="./../builtins/utility-functions.html">Utility-Funktionen Referenz</a></li><li><a href="./system-examples.html">System-Funktionen Beispiele</a></li></ul>`,21)])])}const m=s(l,[["render",i]]);export{d as __pageData,m as default};
