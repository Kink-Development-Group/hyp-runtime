import{_ as s,c as a,o as e,ag as p}from"./chunks/framework.Dli2S8Ej.js";const m=JSON.parse('{"title":"Syntax","description":"","frontmatter":{"sidebar_position":1},"headers":[],"relativePath":"language-reference/syntax.md","filePath":"language-reference/syntax.md","lastUpdated":1750547232000}'),l={name:"language-reference/syntax.md"};function r(i,n,c,u,t,b){return e(),a("div",null,[...n[0]||(n[0]=[p(`<h1 id="syntax" tabindex="-1">Syntax <a class="header-anchor" href="#syntax" aria-label="Permalink to &quot;Syntax&quot;">​</a></h1><p>HypnoScript verwendet eine hypnotische Syntax, die sowohl intuitiv als auch mächtig ist. Lerne die grundlegenden Syntax-Regeln und Konzepte kennen.</p><h2 id="grundstruktur" tabindex="-1">Grundstruktur <a class="header-anchor" href="#grundstruktur" aria-label="Permalink to &quot;Grundstruktur&quot;">​</a></h2><h3 id="programm-struktur" tabindex="-1">Programm-Struktur <a class="header-anchor" href="#programm-struktur" aria-label="Permalink to &quot;Programm-Struktur&quot;">​</a></h3><p>Jedes HypnoScript-Programm beginnt mit <code>Focus</code> und endet mit <code>Relax</code>:</p><div class="language-hyp vp-adaptive-theme line-numbers-mode"><button title="Copy Code" class="copy"></button><span class="lang">hyp</span><pre class="shiki shiki-themes github-light github-dark vp-code" tabindex="0"><code><span class="line"><span>Focus {</span></span>
<span class="line"><span>    // Programm-Code hier</span></span>
<span class="line"><span>} Relax;</span></span></code></pre><div class="line-numbers-wrapper" aria-hidden="true"><span class="line-number">1</span><br><span class="line-number">2</span><br><span class="line-number">3</span><br></div></div><h3 id="entrance-block" tabindex="-1">Entrance-Block <a class="header-anchor" href="#entrance-block" aria-label="Permalink to &quot;Entrance-Block&quot;">​</a></h3><p>Der <code>entrance</code>-Block wird beim Programmstart ausgeführt:</p><div class="language-hyp vp-adaptive-theme line-numbers-mode"><button title="Copy Code" class="copy"></button><span class="lang">hyp</span><pre class="shiki shiki-themes github-light github-dark vp-code" tabindex="0"><code><span class="line"><span>Focus {</span></span>
<span class="line"><span>    entrance {</span></span>
<span class="line"><span>        observe &quot;Programm gestartet&quot;;</span></span>
<span class="line"><span>    }</span></span>
<span class="line"><span>} Relax;</span></span></code></pre><div class="line-numbers-wrapper" aria-hidden="true"><span class="line-number">1</span><br><span class="line-number">2</span><br><span class="line-number">3</span><br><span class="line-number">4</span><br><span class="line-number">5</span><br></div></div><h2 id="variablen-und-zuweisungen" tabindex="-1">Variablen und Zuweisungen <a class="header-anchor" href="#variablen-und-zuweisungen" aria-label="Permalink to &quot;Variablen und Zuweisungen&quot;">​</a></h2><h3 id="induce-variablenzuweisung" tabindex="-1">Induce (Variablenzuweisung) <a class="header-anchor" href="#induce-variablenzuweisung" aria-label="Permalink to &quot;Induce (Variablenzuweisung)&quot;">​</a></h3><p>Verwende <code>induce</code> um Variablen zu erstellen und Werte zuzuweisen:</p><div class="language-hyp vp-adaptive-theme line-numbers-mode"><button title="Copy Code" class="copy"></button><span class="lang">hyp</span><pre class="shiki shiki-themes github-light github-dark vp-code" tabindex="0"><code><span class="line"><span>Focus {</span></span>
<span class="line"><span>    entrance {</span></span>
<span class="line"><span>        induce name = &quot;HypnoScript&quot;;</span></span>
<span class="line"><span>        induce version = 1.0;</span></span>
<span class="line"><span>        induce isActive = true;</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        observe &quot;Name: &quot; + name;</span></span>
<span class="line"><span>        observe &quot;Version: &quot; + version;</span></span>
<span class="line"><span>        observe &quot;Aktiv: &quot; + isActive;</span></span>
<span class="line"><span>    }</span></span>
<span class="line"><span>} Relax;</span></span></code></pre><div class="line-numbers-wrapper" aria-hidden="true"><span class="line-number">1</span><br><span class="line-number">2</span><br><span class="line-number">3</span><br><span class="line-number">4</span><br><span class="line-number">5</span><br><span class="line-number">6</span><br><span class="line-number">7</span><br><span class="line-number">8</span><br><span class="line-number">9</span><br><span class="line-number">10</span><br><span class="line-number">11</span><br></div></div><h3 id="datentypen" tabindex="-1">Datentypen <a class="header-anchor" href="#datentypen" aria-label="Permalink to &quot;Datentypen&quot;">​</a></h3><p>HypnoScript unterstützt verschiedene Datentypen:</p><div class="language-hyp vp-adaptive-theme line-numbers-mode"><button title="Copy Code" class="copy"></button><span class="lang">hyp</span><pre class="shiki shiki-themes github-light github-dark vp-code" tabindex="0"><code><span class="line"><span>Focus {</span></span>
<span class="line"><span>    entrance {</span></span>
<span class="line"><span>        // Strings</span></span>
<span class="line"><span>        induce text = &quot;Hallo Welt&quot;;</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        // Zahlen (Integer und Double)</span></span>
<span class="line"><span>        induce integer = 42;</span></span>
<span class="line"><span>        induce decimal = 3.14159;</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        // Boolean</span></span>
<span class="line"><span>        induce flag = true;</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        // Arrays</span></span>
<span class="line"><span>        induce numbers = [1, 2, 3, 4, 5];</span></span>
<span class="line"><span>        induce names = [&quot;Alice&quot;, &quot;Bob&quot;, &quot;Charlie&quot;];</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        // Records (Objekte)</span></span>
<span class="line"><span>        induce person = {</span></span>
<span class="line"><span>            name: &quot;Max&quot;,</span></span>
<span class="line"><span>            age: 30,</span></span>
<span class="line"><span>            city: &quot;Berlin&quot;</span></span>
<span class="line"><span>        };</span></span>
<span class="line"><span>    }</span></span>
<span class="line"><span>} Relax;</span></span></code></pre><div class="line-numbers-wrapper" aria-hidden="true"><span class="line-number">1</span><br><span class="line-number">2</span><br><span class="line-number">3</span><br><span class="line-number">4</span><br><span class="line-number">5</span><br><span class="line-number">6</span><br><span class="line-number">7</span><br><span class="line-number">8</span><br><span class="line-number">9</span><br><span class="line-number">10</span><br><span class="line-number">11</span><br><span class="line-number">12</span><br><span class="line-number">13</span><br><span class="line-number">14</span><br><span class="line-number">15</span><br><span class="line-number">16</span><br><span class="line-number">17</span><br><span class="line-number">18</span><br><span class="line-number">19</span><br><span class="line-number">20</span><br><span class="line-number">21</span><br><span class="line-number">22</span><br><span class="line-number">23</span><br><span class="line-number">24</span><br></div></div><h2 id="ausgabe" tabindex="-1">Ausgabe <a class="header-anchor" href="#ausgabe" aria-label="Permalink to &quot;Ausgabe&quot;">​</a></h2><h3 id="observe-ausgabe" tabindex="-1">Observe (Ausgabe) <a class="header-anchor" href="#observe-ausgabe" aria-label="Permalink to &quot;Observe (Ausgabe)&quot;">​</a></h3><p>Verwende <code>observe</code> um Text auszugeben:</p><div class="language-hyp vp-adaptive-theme line-numbers-mode"><button title="Copy Code" class="copy"></button><span class="lang">hyp</span><pre class="shiki shiki-themes github-light github-dark vp-code" tabindex="0"><code><span class="line"><span>Focus {</span></span>
<span class="line"><span>    entrance {</span></span>
<span class="line"><span>        observe &quot;Einfache Ausgabe&quot;;</span></span>
<span class="line"><span>        observe &quot;Mehrzeilige&quot; + &quot; &quot; + &quot;Ausgabe&quot;;</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        induce name = &quot;HypnoScript&quot;;</span></span>
<span class="line"><span>        observe &quot;Willkommen bei &quot; + name;</span></span>
<span class="line"><span>    }</span></span>
<span class="line"><span>} Relax;</span></span></code></pre><div class="line-numbers-wrapper" aria-hidden="true"><span class="line-number">1</span><br><span class="line-number">2</span><br><span class="line-number">3</span><br><span class="line-number">4</span><br><span class="line-number">5</span><br><span class="line-number">6</span><br><span class="line-number">7</span><br><span class="line-number">8</span><br><span class="line-number">9</span><br></div></div><h2 id="kontrollstrukturen" tabindex="-1">Kontrollstrukturen <a class="header-anchor" href="#kontrollstrukturen" aria-label="Permalink to &quot;Kontrollstrukturen&quot;">​</a></h2><h3 id="if-else" tabindex="-1">If-Else <a class="header-anchor" href="#if-else" aria-label="Permalink to &quot;If-Else&quot;">​</a></h3><div class="language-hyp vp-adaptive-theme line-numbers-mode"><button title="Copy Code" class="copy"></button><span class="lang">hyp</span><pre class="shiki shiki-themes github-light github-dark vp-code" tabindex="0"><code><span class="line"><span>Focus {</span></span>
<span class="line"><span>    entrance {</span></span>
<span class="line"><span>        induce age = 18;</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        if (age &gt;= 18) {</span></span>
<span class="line"><span>            observe &quot;Volljährig&quot;;</span></span>
<span class="line"><span>        } else {</span></span>
<span class="line"><span>            observe &quot;Minderjährig&quot;;</span></span>
<span class="line"><span>        }</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        // Mit else if</span></span>
<span class="line"><span>        induce score = 85;</span></span>
<span class="line"><span>        if (score &gt;= 90) {</span></span>
<span class="line"><span>            observe &quot;Ausgezeichnet&quot;;</span></span>
<span class="line"><span>        } else if (score &gt;= 80) {</span></span>
<span class="line"><span>            observe &quot;Gut&quot;;</span></span>
<span class="line"><span>        } else if (score &gt;= 70) {</span></span>
<span class="line"><span>            observe &quot;Befriedigend&quot;;</span></span>
<span class="line"><span>        } else {</span></span>
<span class="line"><span>            observe &quot;Verbesserungsbedarf&quot;;</span></span>
<span class="line"><span>        }</span></span>
<span class="line"><span>    }</span></span>
<span class="line"><span>} Relax;</span></span></code></pre><div class="line-numbers-wrapper" aria-hidden="true"><span class="line-number">1</span><br><span class="line-number">2</span><br><span class="line-number">3</span><br><span class="line-number">4</span><br><span class="line-number">5</span><br><span class="line-number">6</span><br><span class="line-number">7</span><br><span class="line-number">8</span><br><span class="line-number">9</span><br><span class="line-number">10</span><br><span class="line-number">11</span><br><span class="line-number">12</span><br><span class="line-number">13</span><br><span class="line-number">14</span><br><span class="line-number">15</span><br><span class="line-number">16</span><br><span class="line-number">17</span><br><span class="line-number">18</span><br><span class="line-number">19</span><br><span class="line-number">20</span><br><span class="line-number">21</span><br><span class="line-number">22</span><br><span class="line-number">23</span><br></div></div><h3 id="while-schleife" tabindex="-1">While-Schleife <a class="header-anchor" href="#while-schleife" aria-label="Permalink to &quot;While-Schleife&quot;">​</a></h3><div class="language-hyp vp-adaptive-theme line-numbers-mode"><button title="Copy Code" class="copy"></button><span class="lang">hyp</span><pre class="shiki shiki-themes github-light github-dark vp-code" tabindex="0"><code><span class="line"><span>Focus {</span></span>
<span class="line"><span>    entrance {</span></span>
<span class="line"><span>        induce counter = 1;</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        while (counter &lt;= 5) {</span></span>
<span class="line"><span>            observe &quot;Zähler: &quot; + counter;</span></span>
<span class="line"><span>            induce counter = counter + 1;</span></span>
<span class="line"><span>        }</span></span>
<span class="line"><span>    }</span></span>
<span class="line"><span>} Relax;</span></span></code></pre><div class="line-numbers-wrapper" aria-hidden="true"><span class="line-number">1</span><br><span class="line-number">2</span><br><span class="line-number">3</span><br><span class="line-number">4</span><br><span class="line-number">5</span><br><span class="line-number">6</span><br><span class="line-number">7</span><br><span class="line-number">8</span><br><span class="line-number">9</span><br><span class="line-number">10</span><br></div></div><h3 id="for-schleife" tabindex="-1">For-Schleife <a class="header-anchor" href="#for-schleife" aria-label="Permalink to &quot;For-Schleife&quot;">​</a></h3><div class="language-hyp vp-adaptive-theme line-numbers-mode"><button title="Copy Code" class="copy"></button><span class="lang">hyp</span><pre class="shiki shiki-themes github-light github-dark vp-code" tabindex="0"><code><span class="line"><span>Focus {</span></span>
<span class="line"><span>    entrance {</span></span>
<span class="line"><span>        // For-Schleife mit Range</span></span>
<span class="line"><span>        for (induce i = 1; i &lt;= 10; induce i = i + 1) {</span></span>
<span class="line"><span>            observe &quot;Iteration &quot; + i;</span></span>
<span class="line"><span>        }</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        // For-Schleife über Array</span></span>
<span class="line"><span>        induce fruits = [&quot;Apfel&quot;, &quot;Banane&quot;, &quot;Orange&quot;];</span></span>
<span class="line"><span>        for (induce i = 0; i &lt; ArrayLength(fruits); induce i = i + 1) {</span></span>
<span class="line"><span>            observe &quot;Frucht &quot; + (i + 1) + &quot;: &quot; + ArrayGet(fruits, i);</span></span>
<span class="line"><span>        }</span></span>
<span class="line"><span>    }</span></span>
<span class="line"><span>} Relax;</span></span></code></pre><div class="line-numbers-wrapper" aria-hidden="true"><span class="line-number">1</span><br><span class="line-number">2</span><br><span class="line-number">3</span><br><span class="line-number">4</span><br><span class="line-number">5</span><br><span class="line-number">6</span><br><span class="line-number">7</span><br><span class="line-number">8</span><br><span class="line-number">9</span><br><span class="line-number">10</span><br><span class="line-number">11</span><br><span class="line-number">12</span><br><span class="line-number">13</span><br><span class="line-number">14</span><br></div></div><h2 id="funktionen" tabindex="-1">Funktionen <a class="header-anchor" href="#funktionen" aria-label="Permalink to &quot;Funktionen&quot;">​</a></h2><h3 id="trance-funktionsdefinition" tabindex="-1">Trance (Funktionsdefinition) <a class="header-anchor" href="#trance-funktionsdefinition" aria-label="Permalink to &quot;Trance (Funktionsdefinition)&quot;">​</a></h3><div class="language-hyp vp-adaptive-theme line-numbers-mode"><button title="Copy Code" class="copy"></button><span class="lang">hyp</span><pre class="shiki shiki-themes github-light github-dark vp-code" tabindex="0"><code><span class="line"><span>Focus {</span></span>
<span class="line"><span>    // Funktion definieren</span></span>
<span class="line"><span>    Trance greet(name) {</span></span>
<span class="line"><span>        observe &quot;Hallo, &quot; + name + &quot;!&quot;;</span></span>
<span class="line"><span>    }</span></span>
<span class="line"><span></span></span>
<span class="line"><span>    Trance add(a, b) {</span></span>
<span class="line"><span>        return a + b;</span></span>
<span class="line"><span>    }</span></span>
<span class="line"><span></span></span>
<span class="line"><span>    Trance factorial(n) {</span></span>
<span class="line"><span>        if (n &lt;= 1) {</span></span>
<span class="line"><span>            return 1;</span></span>
<span class="line"><span>        } else {</span></span>
<span class="line"><span>            return n * factorial(n - 1);</span></span>
<span class="line"><span>        }</span></span>
<span class="line"><span>    }</span></span>
<span class="line"><span></span></span>
<span class="line"><span>    entrance {</span></span>
<span class="line"><span>        // Funktionen aufrufen</span></span>
<span class="line"><span>        greet(&quot;HypnoScript&quot;);</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        induce result = add(5, 3);</span></span>
<span class="line"><span>        observe &quot;5 + 3 = &quot; + result;</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        induce fact = factorial(5);</span></span>
<span class="line"><span>        observe &quot;5! = &quot; + fact;</span></span>
<span class="line"><span>    }</span></span>
<span class="line"><span>} Relax;</span></span></code></pre><div class="line-numbers-wrapper" aria-hidden="true"><span class="line-number">1</span><br><span class="line-number">2</span><br><span class="line-number">3</span><br><span class="line-number">4</span><br><span class="line-number">5</span><br><span class="line-number">6</span><br><span class="line-number">7</span><br><span class="line-number">8</span><br><span class="line-number">9</span><br><span class="line-number">10</span><br><span class="line-number">11</span><br><span class="line-number">12</span><br><span class="line-number">13</span><br><span class="line-number">14</span><br><span class="line-number">15</span><br><span class="line-number">16</span><br><span class="line-number">17</span><br><span class="line-number">18</span><br><span class="line-number">19</span><br><span class="line-number">20</span><br><span class="line-number">21</span><br><span class="line-number">22</span><br><span class="line-number">23</span><br><span class="line-number">24</span><br><span class="line-number">25</span><br><span class="line-number">26</span><br><span class="line-number">27</span><br><span class="line-number">28</span><br><span class="line-number">29</span><br></div></div><h3 id="funktionen-mit-ruckgabewerten" tabindex="-1">Funktionen mit Rückgabewerten <a class="header-anchor" href="#funktionen-mit-ruckgabewerten" aria-label="Permalink to &quot;Funktionen mit Rückgabewerten&quot;">​</a></h3><div class="language-hyp vp-adaptive-theme line-numbers-mode"><button title="Copy Code" class="copy"></button><span class="lang">hyp</span><pre class="shiki shiki-themes github-light github-dark vp-code" tabindex="0"><code><span class="line"><span>Focus {</span></span>
<span class="line"><span>    Trance calculateArea(width, height) {</span></span>
<span class="line"><span>        return width * height;</span></span>
<span class="line"><span>    }</span></span>
<span class="line"><span></span></span>
<span class="line"><span>    Trance isEven(number) {</span></span>
<span class="line"><span>        return number % 2 == 0;</span></span>
<span class="line"><span>    }</span></span>
<span class="line"><span></span></span>
<span class="line"><span>    Trance getMax(a, b) {</span></span>
<span class="line"><span>        if (a &gt; b) {</span></span>
<span class="line"><span>            return a;</span></span>
<span class="line"><span>        } else {</span></span>
<span class="line"><span>            return b;</span></span>
<span class="line"><span>        }</span></span>
<span class="line"><span>    }</span></span>
<span class="line"><span></span></span>
<span class="line"><span>    entrance {</span></span>
<span class="line"><span>        induce area = calculateArea(10, 5);</span></span>
<span class="line"><span>        observe &quot;Fläche: &quot; + area;</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        induce check = isEven(42);</span></span>
<span class="line"><span>        observe &quot;42 ist gerade: &quot; + check;</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        induce maximum = getMax(15, 8);</span></span>
<span class="line"><span>        observe &quot;Maximum: &quot; + maximum;</span></span>
<span class="line"><span>    }</span></span>
<span class="line"><span>} Relax;</span></span></code></pre><div class="line-numbers-wrapper" aria-hidden="true"><span class="line-number">1</span><br><span class="line-number">2</span><br><span class="line-number">3</span><br><span class="line-number">4</span><br><span class="line-number">5</span><br><span class="line-number">6</span><br><span class="line-number">7</span><br><span class="line-number">8</span><br><span class="line-number">9</span><br><span class="line-number">10</span><br><span class="line-number">11</span><br><span class="line-number">12</span><br><span class="line-number">13</span><br><span class="line-number">14</span><br><span class="line-number">15</span><br><span class="line-number">16</span><br><span class="line-number">17</span><br><span class="line-number">18</span><br><span class="line-number">19</span><br><span class="line-number">20</span><br><span class="line-number">21</span><br><span class="line-number">22</span><br><span class="line-number">23</span><br><span class="line-number">24</span><br><span class="line-number">25</span><br><span class="line-number">26</span><br><span class="line-number">27</span><br><span class="line-number">28</span><br></div></div><h2 id="arrays" tabindex="-1">Arrays <a class="header-anchor" href="#arrays" aria-label="Permalink to &quot;Arrays&quot;">​</a></h2><h3 id="array-operationen" tabindex="-1">Array-Operationen <a class="header-anchor" href="#array-operationen" aria-label="Permalink to &quot;Array-Operationen&quot;">​</a></h3><div class="language-hyp vp-adaptive-theme line-numbers-mode"><button title="Copy Code" class="copy"></button><span class="lang">hyp</span><pre class="shiki shiki-themes github-light github-dark vp-code" tabindex="0"><code><span class="line"><span>Focus {</span></span>
<span class="line"><span>    entrance {</span></span>
<span class="line"><span>        // Array erstellen</span></span>
<span class="line"><span>        induce numbers = [1, 2, 3, 4, 5];</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        // Elemente abrufen</span></span>
<span class="line"><span>        induce first = ArrayGet(numbers, 0);</span></span>
<span class="line"><span>        observe &quot;Erstes Element: &quot; + first;</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        // Elemente setzen</span></span>
<span class="line"><span>        ArraySet(numbers, 2, 99);</span></span>
<span class="line"><span>        observe &quot;Nach Änderung: &quot; + numbers;</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        // Array-Länge</span></span>
<span class="line"><span>        induce length = ArrayLength(numbers);</span></span>
<span class="line"><span>        observe &quot;Array-Länge: &quot; + length;</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        // Array durchsuchen</span></span>
<span class="line"><span>        for (induce i = 0; i &lt; ArrayLength(numbers); induce i = i + 1) {</span></span>
<span class="line"><span>            observe &quot;Element &quot; + i + &quot;: &quot; + ArrayGet(numbers, i);</span></span>
<span class="line"><span>        }</span></span>
<span class="line"><span>    }</span></span>
<span class="line"><span>} Relax;</span></span></code></pre><div class="line-numbers-wrapper" aria-hidden="true"><span class="line-number">1</span><br><span class="line-number">2</span><br><span class="line-number">3</span><br><span class="line-number">4</span><br><span class="line-number">5</span><br><span class="line-number">6</span><br><span class="line-number">7</span><br><span class="line-number">8</span><br><span class="line-number">9</span><br><span class="line-number">10</span><br><span class="line-number">11</span><br><span class="line-number">12</span><br><span class="line-number">13</span><br><span class="line-number">14</span><br><span class="line-number">15</span><br><span class="line-number">16</span><br><span class="line-number">17</span><br><span class="line-number">18</span><br><span class="line-number">19</span><br><span class="line-number">20</span><br><span class="line-number">21</span><br><span class="line-number">22</span><br><span class="line-number">23</span><br></div></div><h3 id="array-funktionen" tabindex="-1">Array-Funktionen <a class="header-anchor" href="#array-funktionen" aria-label="Permalink to &quot;Array-Funktionen&quot;">​</a></h3><div class="language-hyp vp-adaptive-theme line-numbers-mode"><button title="Copy Code" class="copy"></button><span class="lang">hyp</span><pre class="shiki shiki-themes github-light github-dark vp-code" tabindex="0"><code><span class="line"><span>Focus {</span></span>
<span class="line"><span>    entrance {</span></span>
<span class="line"><span>        induce numbers = [3, 1, 4, 1, 5, 9, 2, 6];</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        // Sortieren</span></span>
<span class="line"><span>        induce sorted = ArraySort(numbers);</span></span>
<span class="line"><span>        observe &quot;Sortiert: &quot; + sorted;</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        // Summe</span></span>
<span class="line"><span>        induce sum = SumArray(numbers);</span></span>
<span class="line"><span>        observe &quot;Summe: &quot; + sum;</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        // Durchschnitt</span></span>
<span class="line"><span>        induce avg = AverageArray(numbers);</span></span>
<span class="line"><span>        observe &quot;Durchschnitt: &quot; + avg;</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        // Mischen</span></span>
<span class="line"><span>        induce shuffled = ShuffleArray(numbers);</span></span>
<span class="line"><span>        observe &quot;Gemischt: &quot; + shuffled;</span></span>
<span class="line"><span>    }</span></span>
<span class="line"><span>} Relax;</span></span></code></pre><div class="line-numbers-wrapper" aria-hidden="true"><span class="line-number">1</span><br><span class="line-number">2</span><br><span class="line-number">3</span><br><span class="line-number">4</span><br><span class="line-number">5</span><br><span class="line-number">6</span><br><span class="line-number">7</span><br><span class="line-number">8</span><br><span class="line-number">9</span><br><span class="line-number">10</span><br><span class="line-number">11</span><br><span class="line-number">12</span><br><span class="line-number">13</span><br><span class="line-number">14</span><br><span class="line-number">15</span><br><span class="line-number">16</span><br><span class="line-number">17</span><br><span class="line-number">18</span><br><span class="line-number">19</span><br><span class="line-number">20</span><br><span class="line-number">21</span><br></div></div><h2 id="records-objekte" tabindex="-1">Records (Objekte) <a class="header-anchor" href="#records-objekte" aria-label="Permalink to &quot;Records (Objekte)&quot;">​</a></h2><h3 id="record-erstellung-und-zugriff" tabindex="-1">Record-Erstellung und -Zugriff <a class="header-anchor" href="#record-erstellung-und-zugriff" aria-label="Permalink to &quot;Record-Erstellung und -Zugriff&quot;">​</a></h3><div class="language-hyp vp-adaptive-theme line-numbers-mode"><button title="Copy Code" class="copy"></button><span class="lang">hyp</span><pre class="shiki shiki-themes github-light github-dark vp-code" tabindex="0"><code><span class="line"><span>Focus {</span></span>
<span class="line"><span>    entrance {</span></span>
<span class="line"><span>        // Record erstellen</span></span>
<span class="line"><span>        induce person = {</span></span>
<span class="line"><span>            name: &quot;Max Mustermann&quot;,</span></span>
<span class="line"><span>            age: 30,</span></span>
<span class="line"><span>            city: &quot;Berlin&quot;,</span></span>
<span class="line"><span>            hobbies: [&quot;Programmierung&quot;, &quot;Lesen&quot;, &quot;Sport&quot;]</span></span>
<span class="line"><span>        };</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        // Eigenschaften abrufen</span></span>
<span class="line"><span>        observe &quot;Name: &quot; + person.name;</span></span>
<span class="line"><span>        observe &quot;Alter: &quot; + person.age;</span></span>
<span class="line"><span>        observe &quot;Stadt: &quot; + person.city;</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        // Eigenschaften ändern</span></span>
<span class="line"><span>        induce person.age = 31;</span></span>
<span class="line"><span>        observe &quot;Neues Alter: &quot; + person.age;</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        // Verschachtelte Records</span></span>
<span class="line"><span>        induce company = {</span></span>
<span class="line"><span>            name: &quot;HypnoScript GmbH&quot;,</span></span>
<span class="line"><span>            address: {</span></span>
<span class="line"><span>                street: &quot;Musterstraße 123&quot;,</span></span>
<span class="line"><span>                city: &quot;Berlin&quot;,</span></span>
<span class="line"><span>                zip: &quot;10115&quot;</span></span>
<span class="line"><span>            },</span></span>
<span class="line"><span>            employees: [</span></span>
<span class="line"><span>                {name: &quot;Alice&quot;, role: &quot;Developer&quot;},</span></span>
<span class="line"><span>                {name: &quot;Bob&quot;, role: &quot;Designer&quot;}</span></span>
<span class="line"><span>            ]</span></span>
<span class="line"><span>        };</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        observe &quot;Firma: &quot; + company.name;</span></span>
<span class="line"><span>        observe &quot;Adresse: &quot; + company.address.street;</span></span>
<span class="line"><span>        observe &quot;Erster Mitarbeiter: &quot; + company.employees[0].name;</span></span>
<span class="line"><span>    }</span></span>
<span class="line"><span>} Relax;</span></span></code></pre><div class="line-numbers-wrapper" aria-hidden="true"><span class="line-number">1</span><br><span class="line-number">2</span><br><span class="line-number">3</span><br><span class="line-number">4</span><br><span class="line-number">5</span><br><span class="line-number">6</span><br><span class="line-number">7</span><br><span class="line-number">8</span><br><span class="line-number">9</span><br><span class="line-number">10</span><br><span class="line-number">11</span><br><span class="line-number">12</span><br><span class="line-number">13</span><br><span class="line-number">14</span><br><span class="line-number">15</span><br><span class="line-number">16</span><br><span class="line-number">17</span><br><span class="line-number">18</span><br><span class="line-number">19</span><br><span class="line-number">20</span><br><span class="line-number">21</span><br><span class="line-number">22</span><br><span class="line-number">23</span><br><span class="line-number">24</span><br><span class="line-number">25</span><br><span class="line-number">26</span><br><span class="line-number">27</span><br><span class="line-number">28</span><br><span class="line-number">29</span><br><span class="line-number">30</span><br><span class="line-number">31</span><br><span class="line-number">32</span><br><span class="line-number">33</span><br><span class="line-number">34</span><br><span class="line-number">35</span><br><span class="line-number">36</span><br><span class="line-number">37</span><br><span class="line-number">38</span><br></div></div><h2 id="sessions" tabindex="-1">Sessions <a class="header-anchor" href="#sessions" aria-label="Permalink to &quot;Sessions&quot;">​</a></h2><h3 id="session-erstellung" tabindex="-1">Session-Erstellung <a class="header-anchor" href="#session-erstellung" aria-label="Permalink to &quot;Session-Erstellung&quot;">​</a></h3><div class="language-hyp vp-adaptive-theme line-numbers-mode"><button title="Copy Code" class="copy"></button><span class="lang">hyp</span><pre class="shiki shiki-themes github-light github-dark vp-code" tabindex="0"><code><span class="line"><span>Focus {</span></span>
<span class="line"><span>    entrance {</span></span>
<span class="line"><span>        // Session erstellen</span></span>
<span class="line"><span>        induce session = Session(&quot;MeineSession&quot;);</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        // Session-Variablen setzen</span></span>
<span class="line"><span>        SessionSet(session, &quot;user&quot;, &quot;Max&quot;);</span></span>
<span class="line"><span>        SessionSet(session, &quot;level&quot;, 5);</span></span>
<span class="line"><span>        SessionSet(session, &quot;preferences&quot;, {</span></span>
<span class="line"><span>            theme: &quot;dark&quot;,</span></span>
<span class="line"><span>            language: &quot;de&quot;</span></span>
<span class="line"><span>        });</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        // Session-Variablen abrufen</span></span>
<span class="line"><span>        induce user = SessionGet(session, &quot;user&quot;);</span></span>
<span class="line"><span>        induce level = SessionGet(session, &quot;level&quot;);</span></span>
<span class="line"><span>        induce prefs = SessionGet(session, &quot;preferences&quot;);</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        observe &quot;Benutzer: &quot; + user;</span></span>
<span class="line"><span>        observe &quot;Level: &quot; + level;</span></span>
<span class="line"><span>        observe &quot;Theme: &quot; + prefs.theme;</span></span>
<span class="line"><span>    }</span></span>
<span class="line"><span>} Relax;</span></span></code></pre><div class="line-numbers-wrapper" aria-hidden="true"><span class="line-number">1</span><br><span class="line-number">2</span><br><span class="line-number">3</span><br><span class="line-number">4</span><br><span class="line-number">5</span><br><span class="line-number">6</span><br><span class="line-number">7</span><br><span class="line-number">8</span><br><span class="line-number">9</span><br><span class="line-number">10</span><br><span class="line-number">11</span><br><span class="line-number">12</span><br><span class="line-number">13</span><br><span class="line-number">14</span><br><span class="line-number">15</span><br><span class="line-number">16</span><br><span class="line-number">17</span><br><span class="line-number">18</span><br><span class="line-number">19</span><br><span class="line-number">20</span><br><span class="line-number">21</span><br><span class="line-number">22</span><br><span class="line-number">23</span><br></div></div><h2 id="tranceify" tabindex="-1">Tranceify <a class="header-anchor" href="#tranceify" aria-label="Permalink to &quot;Tranceify&quot;">​</a></h2><h3 id="tranceify-fur-hypnotische-anwendungen" tabindex="-1">Tranceify für hypnotische Anwendungen <a class="header-anchor" href="#tranceify-fur-hypnotische-anwendungen" aria-label="Permalink to &quot;Tranceify für hypnotische Anwendungen&quot;">​</a></h3><div class="language-hyp vp-adaptive-theme line-numbers-mode"><button title="Copy Code" class="copy"></button><span class="lang">hyp</span><pre class="shiki shiki-themes github-light github-dark vp-code" tabindex="0"><code><span class="line"><span>Focus {</span></span>
<span class="line"><span>    entrance {</span></span>
<span class="line"><span>        // Tranceify-Session starten</span></span>
<span class="line"><span>        Tranceify(&quot;Entspannung&quot;) {</span></span>
<span class="line"><span>            observe &quot;Du entspannst dich jetzt...&quot;;</span></span>
<span class="line"><span>            observe &quot;Atme tief ein...&quot;;</span></span>
<span class="line"><span>            observe &quot;Und aus...&quot;;</span></span>
<span class="line"><span>            observe &quot;Du fühlst dich ruhig und entspannt...&quot;;</span></span>
<span class="line"><span>        }</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        // Mit Parametern</span></span>
<span class="line"><span>        induce clientName = &quot;Anna&quot;;</span></span>
<span class="line"><span>        Tranceify(&quot;Induktion&quot;, clientName) {</span></span>
<span class="line"><span>            observe &quot;Hallo &quot; + clientName + &quot;, willkommen zu deiner Sitzung...&quot;;</span></span>
<span class="line"><span>            observe &quot;Du bist in einem sicheren Raum...&quot;;</span></span>
<span class="line"><span>            observe &quot;Du kannst dich vollständig entspannen...&quot;;</span></span>
<span class="line"><span>        }</span></span>
<span class="line"><span>    }</span></span>
<span class="line"><span>} Relax;</span></span></code></pre><div class="line-numbers-wrapper" aria-hidden="true"><span class="line-number">1</span><br><span class="line-number">2</span><br><span class="line-number">3</span><br><span class="line-number">4</span><br><span class="line-number">5</span><br><span class="line-number">6</span><br><span class="line-number">7</span><br><span class="line-number">8</span><br><span class="line-number">9</span><br><span class="line-number">10</span><br><span class="line-number">11</span><br><span class="line-number">12</span><br><span class="line-number">13</span><br><span class="line-number">14</span><br><span class="line-number">15</span><br><span class="line-number">16</span><br><span class="line-number">17</span><br><span class="line-number">18</span><br><span class="line-number">19</span><br></div></div><h2 id="imports" tabindex="-1">Imports <a class="header-anchor" href="#imports" aria-label="Permalink to &quot;Imports&quot;">​</a></h2><h3 id="module-importieren" tabindex="-1">Module importieren <a class="header-anchor" href="#module-importieren" aria-label="Permalink to &quot;Module importieren&quot;">​</a></h3><div class="language-hyp vp-adaptive-theme line-numbers-mode"><button title="Copy Code" class="copy"></button><span class="lang">hyp</span><pre class="shiki shiki-themes github-light github-dark vp-code" tabindex="0"><code><span class="line"><span>import &quot;utils.hyp&quot;;</span></span>
<span class="line"><span>import &quot;math.hyp&quot; as MathUtils;</span></span>
<span class="line"><span></span></span>
<span class="line"><span>Focus {</span></span>
<span class="line"><span>    entrance {</span></span>
<span class="line"><span>        // Funktionen aus importierten Modulen verwenden</span></span>
<span class="line"><span>        induce result = MathUtils.calculate(10, 5);</span></span>
<span class="line"><span>        observe &quot;Ergebnis: &quot; + result;</span></span>
<span class="line"><span>    }</span></span>
<span class="line"><span>} Relax;</span></span></code></pre><div class="line-numbers-wrapper" aria-hidden="true"><span class="line-number">1</span><br><span class="line-number">2</span><br><span class="line-number">3</span><br><span class="line-number">4</span><br><span class="line-number">5</span><br><span class="line-number">6</span><br><span class="line-number">7</span><br><span class="line-number">8</span><br><span class="line-number">9</span><br><span class="line-number">10</span><br></div></div><h2 id="assertions" tabindex="-1">Assertions <a class="header-anchor" href="#assertions" aria-label="Permalink to &quot;Assertions&quot;">​</a></h2><h3 id="assertions-fur-tests" tabindex="-1">Assertions für Tests <a class="header-anchor" href="#assertions-fur-tests" aria-label="Permalink to &quot;Assertions für Tests&quot;">​</a></h3><div class="language-hyp vp-adaptive-theme line-numbers-mode"><button title="Copy Code" class="copy"></button><span class="lang">hyp</span><pre class="shiki shiki-themes github-light github-dark vp-code" tabindex="0"><code><span class="line"><span>Focus {</span></span>
<span class="line"><span>    entrance {</span></span>
<span class="line"><span>        induce expected = 10;</span></span>
<span class="line"><span>        induce actual = 5 + 5;</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        // Assertion - Programm stoppt bei Fehler</span></span>
<span class="line"><span>        assert actual == expected : &quot;Erwartet 10, aber erhalten &quot; + actual;</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        observe &quot;Test erfolgreich!&quot;;</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        // Weitere Assertions</span></span>
<span class="line"><span>        induce name = &quot;HypnoScript&quot;;</span></span>
<span class="line"><span>        assert Length(name) &gt; 0 : &quot;Name darf nicht leer sein&quot;;</span></span>
<span class="line"><span>        assert Length(name) &lt;= 50 : &quot;Name zu lang&quot;;</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        observe &quot;Alle Tests bestanden!&quot;;</span></span>
<span class="line"><span>    }</span></span>
<span class="line"><span>} Relax;</span></span></code></pre><div class="line-numbers-wrapper" aria-hidden="true"><span class="line-number">1</span><br><span class="line-number">2</span><br><span class="line-number">3</span><br><span class="line-number">4</span><br><span class="line-number">5</span><br><span class="line-number">6</span><br><span class="line-number">7</span><br><span class="line-number">8</span><br><span class="line-number">9</span><br><span class="line-number">10</span><br><span class="line-number">11</span><br><span class="line-number">12</span><br><span class="line-number">13</span><br><span class="line-number">14</span><br><span class="line-number">15</span><br><span class="line-number">16</span><br><span class="line-number">17</span><br><span class="line-number">18</span><br></div></div><h2 id="kommentare" tabindex="-1">Kommentare <a class="header-anchor" href="#kommentare" aria-label="Permalink to &quot;Kommentare&quot;">​</a></h2><h3 id="kommentare-in-hypnoscript" tabindex="-1">Kommentare in HypnoScript <a class="header-anchor" href="#kommentare-in-hypnoscript" aria-label="Permalink to &quot;Kommentare in HypnoScript&quot;">​</a></h3><div class="language-hyp vp-adaptive-theme line-numbers-mode"><button title="Copy Code" class="copy"></button><span class="lang">hyp</span><pre class="shiki shiki-themes github-light github-dark vp-code" tabindex="0"><code><span class="line"><span>Focus {</span></span>
<span class="line"><span>    // Einzeiliger Kommentar</span></span>
<span class="line"><span></span></span>
<span class="line"><span>    entrance {</span></span>
<span class="line"><span>        induce name = &quot;HypnoScript&quot;; // Inline-Kommentar</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        /*</span></span>
<span class="line"><span>         * Mehrzeiliger Kommentar</span></span>
<span class="line"><span>         * Kann über mehrere Zeilen gehen</span></span>
<span class="line"><span>         * Nützlich für längere Erklärungen</span></span>
<span class="line"><span>         */</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        observe &quot;Hallo &quot; + name;</span></span>
<span class="line"><span>    }</span></span>
<span class="line"><span>} Relax;</span></span></code></pre><div class="line-numbers-wrapper" aria-hidden="true"><span class="line-number">1</span><br><span class="line-number">2</span><br><span class="line-number">3</span><br><span class="line-number">4</span><br><span class="line-number">5</span><br><span class="line-number">6</span><br><span class="line-number">7</span><br><span class="line-number">8</span><br><span class="line-number">9</span><br><span class="line-number">10</span><br><span class="line-number">11</span><br><span class="line-number">12</span><br><span class="line-number">13</span><br><span class="line-number">14</span><br><span class="line-number">15</span><br></div></div><h2 id="operatoren" tabindex="-1">Operatoren <a class="header-anchor" href="#operatoren" aria-label="Permalink to &quot;Operatoren&quot;">​</a></h2><h3 id="arithmetische-operatoren" tabindex="-1">Arithmetische Operatoren <a class="header-anchor" href="#arithmetische-operatoren" aria-label="Permalink to &quot;Arithmetische Operatoren&quot;">​</a></h3><div class="language-hyp vp-adaptive-theme line-numbers-mode"><button title="Copy Code" class="copy"></button><span class="lang">hyp</span><pre class="shiki shiki-themes github-light github-dark vp-code" tabindex="0"><code><span class="line"><span>Focus {</span></span>
<span class="line"><span>    entrance {</span></span>
<span class="line"><span>        induce a = 10;</span></span>
<span class="line"><span>        induce b = 3;</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        observe &quot;Addition: &quot; + (a + b);        // 13</span></span>
<span class="line"><span>        observe &quot;Subtraktion: &quot; + (a - b);     // 7</span></span>
<span class="line"><span>        observe &quot;Multiplikation: &quot; + (a * b);  // 30</span></span>
<span class="line"><span>        observe &quot;Division: &quot; + (a / b);        // 3.333...</span></span>
<span class="line"><span>        observe &quot;Modulo: &quot; + (a % b);          // 1</span></span>
<span class="line"><span>        observe &quot;Potenz: &quot; + (a ^ b);          // 1000</span></span>
<span class="line"><span>    }</span></span>
<span class="line"><span>} Relax;</span></span></code></pre><div class="line-numbers-wrapper" aria-hidden="true"><span class="line-number">1</span><br><span class="line-number">2</span><br><span class="line-number">3</span><br><span class="line-number">4</span><br><span class="line-number">5</span><br><span class="line-number">6</span><br><span class="line-number">7</span><br><span class="line-number">8</span><br><span class="line-number">9</span><br><span class="line-number">10</span><br><span class="line-number">11</span><br><span class="line-number">12</span><br><span class="line-number">13</span><br></div></div><h3 id="vergleichsoperatoren" tabindex="-1">Vergleichsoperatoren <a class="header-anchor" href="#vergleichsoperatoren" aria-label="Permalink to &quot;Vergleichsoperatoren&quot;">​</a></h3><div class="language-hyp vp-adaptive-theme line-numbers-mode"><button title="Copy Code" class="copy"></button><span class="lang">hyp</span><pre class="shiki shiki-themes github-light github-dark vp-code" tabindex="0"><code><span class="line"><span>Focus {</span></span>
<span class="line"><span>    entrance {</span></span>
<span class="line"><span>        induce x = 5;</span></span>
<span class="line"><span>        induce y = 10;</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        observe &quot;Gleich: &quot; + (x == y);         // false</span></span>
<span class="line"><span>        observe &quot;Ungleich: &quot; + (x != y);       // true</span></span>
<span class="line"><span>        observe &quot;Kleiner: &quot; + (x &lt; y);         // true</span></span>
<span class="line"><span>        observe &quot;Größer: &quot; + (x &gt; y);          // false</span></span>
<span class="line"><span>        observe &quot;Kleiner gleich: &quot; + (x &lt;= y); // true</span></span>
<span class="line"><span>        observe &quot;Größer gleich: &quot; + (x &gt;= y);  // false</span></span>
<span class="line"><span>    }</span></span>
<span class="line"><span>} Relax;</span></span></code></pre><div class="line-numbers-wrapper" aria-hidden="true"><span class="line-number">1</span><br><span class="line-number">2</span><br><span class="line-number">3</span><br><span class="line-number">4</span><br><span class="line-number">5</span><br><span class="line-number">6</span><br><span class="line-number">7</span><br><span class="line-number">8</span><br><span class="line-number">9</span><br><span class="line-number">10</span><br><span class="line-number">11</span><br><span class="line-number">12</span><br><span class="line-number">13</span><br></div></div><h3 id="logische-operatoren" tabindex="-1">Logische Operatoren <a class="header-anchor" href="#logische-operatoren" aria-label="Permalink to &quot;Logische Operatoren&quot;">​</a></h3><div class="language-hyp vp-adaptive-theme line-numbers-mode"><button title="Copy Code" class="copy"></button><span class="lang">hyp</span><pre class="shiki shiki-themes github-light github-dark vp-code" tabindex="0"><code><span class="line"><span>Focus {</span></span>
<span class="line"><span>    entrance {</span></span>
<span class="line"><span>        induce a = true;</span></span>
<span class="line"><span>        induce b = false;</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        observe &quot;UND: &quot; + (a &amp;&amp; b);            // false</span></span>
<span class="line"><span>        observe &quot;ODER: &quot; + (a || b);           // true</span></span>
<span class="line"><span>        observe &quot;NICHT: &quot; + (!a);              // false</span></span>
<span class="line"><span>        observe &quot;XOR: &quot; + (a ^ b);             // true</span></span>
<span class="line"><span>    }</span></span>
<span class="line"><span>} Relax;</span></span></code></pre><div class="line-numbers-wrapper" aria-hidden="true"><span class="line-number">1</span><br><span class="line-number">2</span><br><span class="line-number">3</span><br><span class="line-number">4</span><br><span class="line-number">5</span><br><span class="line-number">6</span><br><span class="line-number">7</span><br><span class="line-number">8</span><br><span class="line-number">9</span><br><span class="line-number">10</span><br><span class="line-number">11</span><br></div></div><h2 id="best-practices" tabindex="-1">Best Practices <a class="header-anchor" href="#best-practices" aria-label="Permalink to &quot;Best Practices&quot;">​</a></h2><h3 id="code-formatierung" tabindex="-1">Code-Formatierung <a class="header-anchor" href="#code-formatierung" aria-label="Permalink to &quot;Code-Formatierung&quot;">​</a></h3><div class="language-hyp vp-adaptive-theme line-numbers-mode"><button title="Copy Code" class="copy"></button><span class="lang">hyp</span><pre class="shiki shiki-themes github-light github-dark vp-code" tabindex="0"><code><span class="line"><span>Focus {</span></span>
<span class="line"><span>    // Funktionen am Anfang definieren</span></span>
<span class="line"><span>    Trance calculateSum(a, b) {</span></span>
<span class="line"><span>        return a + b;</span></span>
<span class="line"><span>    }</span></span>
<span class="line"><span></span></span>
<span class="line"><span>    Trance validateInput(value) {</span></span>
<span class="line"><span>        return value &gt; 0 &amp;&amp; value &lt;= 100;</span></span>
<span class="line"><span>    }</span></span>
<span class="line"><span></span></span>
<span class="line"><span>    entrance {</span></span>
<span class="line"><span>        // Hauptlogik im entrance-Block</span></span>
<span class="line"><span>        induce input = 42;</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        if (validateInput(input)) {</span></span>
<span class="line"><span>            induce result = calculateSum(input, 10);</span></span>
<span class="line"><span>            observe &quot;Ergebnis: &quot; + result;</span></span>
<span class="line"><span>        } else {</span></span>
<span class="line"><span>            observe &quot;Ungültige Eingabe&quot;;</span></span>
<span class="line"><span>        }</span></span>
<span class="line"><span>    }</span></span>
<span class="line"><span>} Relax;</span></span></code></pre><div class="line-numbers-wrapper" aria-hidden="true"><span class="line-number">1</span><br><span class="line-number">2</span><br><span class="line-number">3</span><br><span class="line-number">4</span><br><span class="line-number">5</span><br><span class="line-number">6</span><br><span class="line-number">7</span><br><span class="line-number">8</span><br><span class="line-number">9</span><br><span class="line-number">10</span><br><span class="line-number">11</span><br><span class="line-number">12</span><br><span class="line-number">13</span><br><span class="line-number">14</span><br><span class="line-number">15</span><br><span class="line-number">16</span><br><span class="line-number">17</span><br><span class="line-number">18</span><br><span class="line-number">19</span><br><span class="line-number">20</span><br><span class="line-number">21</span><br><span class="line-number">22</span><br></div></div><h3 id="namenskonventionen" tabindex="-1">Namenskonventionen <a class="header-anchor" href="#namenskonventionen" aria-label="Permalink to &quot;Namenskonventionen&quot;">​</a></h3><ul><li><strong>Variablen</strong>: camelCase (<code>userName</code>, <code>totalCount</code>)</li><li><strong>Funktionen</strong>: camelCase (<code>calculateArea</code>, <code>validateInput</code>)</li><li><strong>Konstanten</strong>: UPPER_SNAKE_CASE (<code>MAX_RETRY_COUNT</code>)</li><li><strong>Sessions</strong>: PascalCase (<code>UserSession</code>, <code>GameState</code>)</li></ul><h3 id="fehlerbehandlung" tabindex="-1">Fehlerbehandlung <a class="header-anchor" href="#fehlerbehandlung" aria-label="Permalink to &quot;Fehlerbehandlung&quot;">​</a></h3><div class="language-hyp vp-adaptive-theme line-numbers-mode"><button title="Copy Code" class="copy"></button><span class="lang">hyp</span><pre class="shiki shiki-themes github-light github-dark vp-code" tabindex="0"><code><span class="line"><span>Focus {</span></span>
<span class="line"><span>    entrance {</span></span>
<span class="line"><span>        induce input = &quot;abc&quot;;</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        // Typprüfung</span></span>
<span class="line"><span>        if (IsNumber(input)) {</span></span>
<span class="line"><span>            induce number = ToNumber(input);</span></span>
<span class="line"><span>            observe &quot;Zahl: &quot; + number;</span></span>
<span class="line"><span>        } else {</span></span>
<span class="line"><span>            observe &quot;Fehler: Keine gültige Zahl&quot;;</span></span>
<span class="line"><span>        }</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        // Array-Zugriff prüfen</span></span>
<span class="line"><span>        induce array = [1, 2, 3];</span></span>
<span class="line"><span>        induce index = 5;</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        if (index &gt;= 0 &amp;&amp; index &lt; ArrayLength(array)) {</span></span>
<span class="line"><span>            induce value = ArrayGet(array, index);</span></span>
<span class="line"><span>            observe &quot;Wert: &quot; + value;</span></span>
<span class="line"><span>        } else {</span></span>
<span class="line"><span>            observe &quot;Fehler: Index außerhalb des Bereichs&quot;;</span></span>
<span class="line"><span>        }</span></span>
<span class="line"><span>    }</span></span>
<span class="line"><span>} Relax;</span></span></code></pre><div class="line-numbers-wrapper" aria-hidden="true"><span class="line-number">1</span><br><span class="line-number">2</span><br><span class="line-number">3</span><br><span class="line-number">4</span><br><span class="line-number">5</span><br><span class="line-number">6</span><br><span class="line-number">7</span><br><span class="line-number">8</span><br><span class="line-number">9</span><br><span class="line-number">10</span><br><span class="line-number">11</span><br><span class="line-number">12</span><br><span class="line-number">13</span><br><span class="line-number">14</span><br><span class="line-number">15</span><br><span class="line-number">16</span><br><span class="line-number">17</span><br><span class="line-number">18</span><br><span class="line-number">19</span><br><span class="line-number">20</span><br><span class="line-number">21</span><br><span class="line-number">22</span><br><span class="line-number">23</span><br><span class="line-number">24</span><br></div></div><h2 id="nachste-schritte" tabindex="-1">Nächste Schritte <a class="header-anchor" href="#nachste-schritte" aria-label="Permalink to &quot;Nächste Schritte&quot;">​</a></h2><ul><li><a href="./variables.html">Variablen und Datentypen</a> - Detaillierte Informationen zu Variablen</li><li><a href="./operators.html">Operatoren</a> - Alle verfügbaren Operatoren</li><li><a href="./control-flow.html">Kontrollstrukturen</a> - If, While, For und mehr</li><li><a href="./functions.html">Funktionen</a> - Funktionsdefinition und -aufruf</li><li><a href="./../examples/basic-examples.html">Beispiele</a> - Praktische Beispiele</li></ul><hr><p><strong>Beherrschst du die Grundlagen? Dann lerne mehr über <a href="./variables.html">Variablen und Datentypen</a>!</strong> 📚</p>`,73)])])}const d=s(l,[["render",r]]);export{m as __pageData,d as default};
