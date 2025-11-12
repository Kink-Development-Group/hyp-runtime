import{_ as s,c as a,o as e,ag as p}from"./chunks/framework.Dli2S8Ej.js";const h=JSON.parse('{"title":"Kontrollstrukturen","description":"","frontmatter":{"sidebar_position":4},"headers":[],"relativePath":"language-reference/control-flow.md","filePath":"language-reference/control-flow.md","lastUpdated":1750547232000}'),l={name:"language-reference/control-flow.md"};function i(r,n,c,u,t,b){return e(),a("div",null,[...n[0]||(n[0]=[p(`<h1 id="kontrollstrukturen" tabindex="-1">Kontrollstrukturen <a class="header-anchor" href="#kontrollstrukturen" aria-label="Permalink to &quot;Kontrollstrukturen&quot;">​</a></h1><p>HypnoScript bietet verschiedene Kontrollstrukturen für bedingte Ausführung und Schleifen.</p><h2 id="if-else-anweisungen" tabindex="-1">If-Else Anweisungen <a class="header-anchor" href="#if-else-anweisungen" aria-label="Permalink to &quot;If-Else Anweisungen&quot;">​</a></h2><h3 id="einfache-if-anweisung" tabindex="-1">Einfache If-Anweisung <a class="header-anchor" href="#einfache-if-anweisung" aria-label="Permalink to &quot;Einfache If-Anweisung&quot;">​</a></h3><div class="language-hyp vp-adaptive-theme line-numbers-mode"><button title="Copy Code" class="copy"></button><span class="lang">hyp</span><pre class="shiki shiki-themes github-light github-dark vp-code" tabindex="0"><code><span class="line"><span>if (bedingung) {</span></span>
<span class="line"><span>    // Code wird ausgeführt, wenn bedingung true ist</span></span>
<span class="line"><span>}</span></span></code></pre><div class="line-numbers-wrapper" aria-hidden="true"><span class="line-number">1</span><br><span class="line-number">2</span><br><span class="line-number">3</span><br></div></div><h3 id="if-else-anweisung" tabindex="-1">If-Else Anweisung <a class="header-anchor" href="#if-else-anweisung" aria-label="Permalink to &quot;If-Else Anweisung&quot;">​</a></h3><div class="language-hyp vp-adaptive-theme line-numbers-mode"><button title="Copy Code" class="copy"></button><span class="lang">hyp</span><pre class="shiki shiki-themes github-light github-dark vp-code" tabindex="0"><code><span class="line"><span>if (bedingung) {</span></span>
<span class="line"><span>    // Code wenn bedingung true ist</span></span>
<span class="line"><span>} else {</span></span>
<span class="line"><span>    // Code wenn bedingung false ist</span></span>
<span class="line"><span>}</span></span></code></pre><div class="line-numbers-wrapper" aria-hidden="true"><span class="line-number">1</span><br><span class="line-number">2</span><br><span class="line-number">3</span><br><span class="line-number">4</span><br><span class="line-number">5</span><br></div></div><h3 id="if-else-if-else-anweisung" tabindex="-1">If-Else If-Else Anweisung <a class="header-anchor" href="#if-else-if-else-anweisung" aria-label="Permalink to &quot;If-Else If-Else Anweisung&quot;">​</a></h3><div class="language-hyp vp-adaptive-theme line-numbers-mode"><button title="Copy Code" class="copy"></button><span class="lang">hyp</span><pre class="shiki shiki-themes github-light github-dark vp-code" tabindex="0"><code><span class="line"><span>if (bedingung1) {</span></span>
<span class="line"><span>    // Code wenn bedingung1 true ist</span></span>
<span class="line"><span>} else if (bedingung2) {</span></span>
<span class="line"><span>    // Code wenn bedingung2 true ist</span></span>
<span class="line"><span>} else {</span></span>
<span class="line"><span>    // Code wenn alle bedingungen false sind</span></span>
<span class="line"><span>}</span></span></code></pre><div class="line-numbers-wrapper" aria-hidden="true"><span class="line-number">1</span><br><span class="line-number">2</span><br><span class="line-number">3</span><br><span class="line-number">4</span><br><span class="line-number">5</span><br><span class="line-number">6</span><br><span class="line-number">7</span><br></div></div><h3 id="beispiele" tabindex="-1">Beispiele <a class="header-anchor" href="#beispiele" aria-label="Permalink to &quot;Beispiele&quot;">​</a></h3><div class="language-hyp vp-adaptive-theme line-numbers-mode"><button title="Copy Code" class="copy"></button><span class="lang">hyp</span><pre class="shiki shiki-themes github-light github-dark vp-code" tabindex="0"><code><span class="line"><span>Focus {</span></span>
<span class="line"><span>    entrance {</span></span>
<span class="line"><span>        induce alter = 18;</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        if (alter &gt;= 18) {</span></span>
<span class="line"><span>            observe &quot;Volljährig&quot;;</span></span>
<span class="line"><span>        } else {</span></span>
<span class="line"><span>            observe &quot;Minderjährig&quot;;</span></span>
<span class="line"><span>        }</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        induce punktzahl = 85;</span></span>
<span class="line"><span>        if (punktzahl &gt;= 90) {</span></span>
<span class="line"><span>            observe &quot;Ausgezeichnet&quot;;</span></span>
<span class="line"><span>        } else if (punktzahl &gt;= 80) {</span></span>
<span class="line"><span>            observe &quot;Gut&quot;;</span></span>
<span class="line"><span>        } else if (punktzahl &gt;= 70) {</span></span>
<span class="line"><span>            observe &quot;Befriedigend&quot;;</span></span>
<span class="line"><span>        } else {</span></span>
<span class="line"><span>            observe &quot;Verbesserungsbedarf&quot;;</span></span>
<span class="line"><span>        }</span></span>
<span class="line"><span>    }</span></span>
<span class="line"><span>} Relax;</span></span></code></pre><div class="line-numbers-wrapper" aria-hidden="true"><span class="line-number">1</span><br><span class="line-number">2</span><br><span class="line-number">3</span><br><span class="line-number">4</span><br><span class="line-number">5</span><br><span class="line-number">6</span><br><span class="line-number">7</span><br><span class="line-number">8</span><br><span class="line-number">9</span><br><span class="line-number">10</span><br><span class="line-number">11</span><br><span class="line-number">12</span><br><span class="line-number">13</span><br><span class="line-number">14</span><br><span class="line-number">15</span><br><span class="line-number">16</span><br><span class="line-number">17</span><br><span class="line-number">18</span><br><span class="line-number">19</span><br><span class="line-number">20</span><br><span class="line-number">21</span><br><span class="line-number">22</span><br></div></div><h2 id="while-schleifen" tabindex="-1">While-Schleifen <a class="header-anchor" href="#while-schleifen" aria-label="Permalink to &quot;While-Schleifen&quot;">​</a></h2><h3 id="syntax" tabindex="-1">Syntax <a class="header-anchor" href="#syntax" aria-label="Permalink to &quot;Syntax&quot;">​</a></h3><div class="language-hyp vp-adaptive-theme line-numbers-mode"><button title="Copy Code" class="copy"></button><span class="lang">hyp</span><pre class="shiki shiki-themes github-light github-dark vp-code" tabindex="0"><code><span class="line"><span>while (bedingung) {</span></span>
<span class="line"><span>    // Code wird wiederholt, solange bedingung true ist</span></span>
<span class="line"><span>}</span></span></code></pre><div class="line-numbers-wrapper" aria-hidden="true"><span class="line-number">1</span><br><span class="line-number">2</span><br><span class="line-number">3</span><br></div></div><h3 id="beispiele-1" tabindex="-1">Beispiele <a class="header-anchor" href="#beispiele-1" aria-label="Permalink to &quot;Beispiele&quot;">​</a></h3><div class="language-hyp vp-adaptive-theme line-numbers-mode"><button title="Copy Code" class="copy"></button><span class="lang">hyp</span><pre class="shiki shiki-themes github-light github-dark vp-code" tabindex="0"><code><span class="line"><span>Focus {</span></span>
<span class="line"><span>    entrance {</span></span>
<span class="line"><span>        // Einfache While-Schleife</span></span>
<span class="line"><span>        induce zaehler = 1;</span></span>
<span class="line"><span>        while (zaehler &lt;= 5) {</span></span>
<span class="line"><span>            observe &quot;Zähler: &quot; + zaehler;</span></span>
<span class="line"><span>            induce zaehler = zaehler + 1;</span></span>
<span class="line"><span>        }</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        // While-Schleife mit Array</span></span>
<span class="line"><span>        induce zahlen = [1, 2, 3, 4, 5];</span></span>
<span class="line"><span>        induce index = 0;</span></span>
<span class="line"><span>        while (index &lt; ArrayLength(zahlen)) {</span></span>
<span class="line"><span>            observe &quot;Zahl &quot; + (index + 1) + &quot;: &quot; + ArrayGet(zahlen, index);</span></span>
<span class="line"><span>            induce index = index + 1;</span></span>
<span class="line"><span>        }</span></span>
<span class="line"><span>    }</span></span>
<span class="line"><span>} Relax;</span></span></code></pre><div class="line-numbers-wrapper" aria-hidden="true"><span class="line-number">1</span><br><span class="line-number">2</span><br><span class="line-number">3</span><br><span class="line-number">4</span><br><span class="line-number">5</span><br><span class="line-number">6</span><br><span class="line-number">7</span><br><span class="line-number">8</span><br><span class="line-number">9</span><br><span class="line-number">10</span><br><span class="line-number">11</span><br><span class="line-number">12</span><br><span class="line-number">13</span><br><span class="line-number">14</span><br><span class="line-number">15</span><br><span class="line-number">16</span><br><span class="line-number">17</span><br><span class="line-number">18</span><br></div></div><h2 id="for-schleifen" tabindex="-1">For-Schleifen <a class="header-anchor" href="#for-schleifen" aria-label="Permalink to &quot;For-Schleifen&quot;">​</a></h2><h3 id="syntax-1" tabindex="-1">Syntax <a class="header-anchor" href="#syntax-1" aria-label="Permalink to &quot;Syntax&quot;">​</a></h3><div class="language-hyp vp-adaptive-theme line-numbers-mode"><button title="Copy Code" class="copy"></button><span class="lang">hyp</span><pre class="shiki shiki-themes github-light github-dark vp-code" tabindex="0"><code><span class="line"><span>for (initialisierung; bedingung; inkrement) {</span></span>
<span class="line"><span>    // Code wird wiederholt</span></span>
<span class="line"><span>}</span></span></code></pre><div class="line-numbers-wrapper" aria-hidden="true"><span class="line-number">1</span><br><span class="line-number">2</span><br><span class="line-number">3</span><br></div></div><h3 id="beispiele-2" tabindex="-1">Beispiele <a class="header-anchor" href="#beispiele-2" aria-label="Permalink to &quot;Beispiele&quot;">​</a></h3><div class="language-hyp vp-adaptive-theme line-numbers-mode"><button title="Copy Code" class="copy"></button><span class="lang">hyp</span><pre class="shiki shiki-themes github-light github-dark vp-code" tabindex="0"><code><span class="line"><span>Focus {</span></span>
<span class="line"><span>    entrance {</span></span>
<span class="line"><span>        // Standard For-Schleife</span></span>
<span class="line"><span>        for (induce i = 1; i &lt;= 10; induce i = i + 1) {</span></span>
<span class="line"><span>            observe &quot;Iteration &quot; + i;</span></span>
<span class="line"><span>        }</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        // For-Schleife über Array</span></span>
<span class="line"><span>        induce obst = [&quot;Apfel&quot;, &quot;Banane&quot;, &quot;Orange&quot;];</span></span>
<span class="line"><span>        for (induce i = 0; i &lt; ArrayLength(obst); induce i = i + 1) {</span></span>
<span class="line"><span>            observe &quot;Obst &quot; + (i + 1) + &quot;: &quot; + ArrayGet(obst, i);</span></span>
<span class="line"><span>        }</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        // Rückwärts zählen</span></span>
<span class="line"><span>        for (induce i = 10; i &gt;= 1; induce i = i - 1) {</span></span>
<span class="line"><span>            observe &quot;Countdown: &quot; + i;</span></span>
<span class="line"><span>        }</span></span>
<span class="line"><span>    }</span></span>
<span class="line"><span>} Relax;</span></span></code></pre><div class="line-numbers-wrapper" aria-hidden="true"><span class="line-number">1</span><br><span class="line-number">2</span><br><span class="line-number">3</span><br><span class="line-number">4</span><br><span class="line-number">5</span><br><span class="line-number">6</span><br><span class="line-number">7</span><br><span class="line-number">8</span><br><span class="line-number">9</span><br><span class="line-number">10</span><br><span class="line-number">11</span><br><span class="line-number">12</span><br><span class="line-number">13</span><br><span class="line-number">14</span><br><span class="line-number">15</span><br><span class="line-number">16</span><br><span class="line-number">17</span><br><span class="line-number">18</span><br><span class="line-number">19</span><br></div></div><h2 id="verschachtelte-kontrollstrukturen" tabindex="-1">Verschachtelte Kontrollstrukturen <a class="header-anchor" href="#verschachtelte-kontrollstrukturen" aria-label="Permalink to &quot;Verschachtelte Kontrollstrukturen&quot;">​</a></h2><div class="language-hyp vp-adaptive-theme line-numbers-mode"><button title="Copy Code" class="copy"></button><span class="lang">hyp</span><pre class="shiki shiki-themes github-light github-dark vp-code" tabindex="0"><code><span class="line"><span>Focus {</span></span>
<span class="line"><span>    entrance {</span></span>
<span class="line"><span>        induce zahlen = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        for (induce i = 0; i &lt; ArrayLength(zahlen); induce i = i + 1) {</span></span>
<span class="line"><span>            induce zahl = ArrayGet(zahlen, i);</span></span>
<span class="line"><span></span></span>
<span class="line"><span>            if (zahl % 2 == 0) {</span></span>
<span class="line"><span>                observe zahl + &quot; ist gerade&quot;;</span></span>
<span class="line"><span>            } else {</span></span>
<span class="line"><span>                observe zahl + &quot; ist ungerade&quot;;</span></span>
<span class="line"><span>            }</span></span>
<span class="line"><span></span></span>
<span class="line"><span>            if (zahl &lt; 5) {</span></span>
<span class="line"><span>                observe &quot;  - Kleine Zahl&quot;;</span></span>
<span class="line"><span>            } else if (zahl &lt; 8) {</span></span>
<span class="line"><span>                observe &quot;  - Mittlere Zahl&quot;;</span></span>
<span class="line"><span>            } else {</span></span>
<span class="line"><span>                observe &quot;  - Große Zahl&quot;;</span></span>
<span class="line"><span>            }</span></span>
<span class="line"><span>        }</span></span>
<span class="line"><span>    }</span></span>
<span class="line"><span>} Relax;</span></span></code></pre><div class="line-numbers-wrapper" aria-hidden="true"><span class="line-number">1</span><br><span class="line-number">2</span><br><span class="line-number">3</span><br><span class="line-number">4</span><br><span class="line-number">5</span><br><span class="line-number">6</span><br><span class="line-number">7</span><br><span class="line-number">8</span><br><span class="line-number">9</span><br><span class="line-number">10</span><br><span class="line-number">11</span><br><span class="line-number">12</span><br><span class="line-number">13</span><br><span class="line-number">14</span><br><span class="line-number">15</span><br><span class="line-number">16</span><br><span class="line-number">17</span><br><span class="line-number">18</span><br><span class="line-number">19</span><br><span class="line-number">20</span><br><span class="line-number">21</span><br><span class="line-number">22</span><br><span class="line-number">23</span><br></div></div><h2 id="break-und-continue" tabindex="-1">Break und Continue <a class="header-anchor" href="#break-und-continue" aria-label="Permalink to &quot;Break und Continue&quot;">​</a></h2><h3 id="break" tabindex="-1">Break <a class="header-anchor" href="#break" aria-label="Permalink to &quot;Break&quot;">​</a></h3><p>Beendet die aktuelle Schleife sofort:</p><div class="language-hyp vp-adaptive-theme line-numbers-mode"><button title="Copy Code" class="copy"></button><span class="lang">hyp</span><pre class="shiki shiki-themes github-light github-dark vp-code" tabindex="0"><code><span class="line"><span>Focus {</span></span>
<span class="line"><span>    entrance {</span></span>
<span class="line"><span>        for (induce i = 1; i &lt;= 10; induce i = i + 1) {</span></span>
<span class="line"><span>            if (i == 5) {</span></span>
<span class="line"><span>                break; // Schleife wird bei i=5 beendet</span></span>
<span class="line"><span>            }</span></span>
<span class="line"><span>            observe &quot;Zahl: &quot; + i;</span></span>
<span class="line"><span>        }</span></span>
<span class="line"><span>        observe &quot;Schleife beendet&quot;;</span></span>
<span class="line"><span>    }</span></span>
<span class="line"><span>} Relax;</span></span></code></pre><div class="line-numbers-wrapper" aria-hidden="true"><span class="line-number">1</span><br><span class="line-number">2</span><br><span class="line-number">3</span><br><span class="line-number">4</span><br><span class="line-number">5</span><br><span class="line-number">6</span><br><span class="line-number">7</span><br><span class="line-number">8</span><br><span class="line-number">9</span><br><span class="line-number">10</span><br><span class="line-number">11</span><br></div></div><h3 id="continue" tabindex="-1">Continue <a class="header-anchor" href="#continue" aria-label="Permalink to &quot;Continue&quot;">​</a></h3><p>Überspringt den aktuellen Schleifendurchlauf:</p><div class="language-hyp vp-adaptive-theme line-numbers-mode"><button title="Copy Code" class="copy"></button><span class="lang">hyp</span><pre class="shiki shiki-themes github-light github-dark vp-code" tabindex="0"><code><span class="line"><span>Focus {</span></span>
<span class="line"><span>    entrance {</span></span>
<span class="line"><span>        for (induce i = 1; i &lt;= 10; induce i = i + 1) {</span></span>
<span class="line"><span>            if (i % 2 == 0) {</span></span>
<span class="line"><span>                continue; // Gerade Zahlen werden übersprungen</span></span>
<span class="line"><span>            }</span></span>
<span class="line"><span>            observe &quot;Ungerade Zahl: &quot; + i;</span></span>
<span class="line"><span>        }</span></span>
<span class="line"><span>    }</span></span>
<span class="line"><span>} Relax;</span></span></code></pre><div class="line-numbers-wrapper" aria-hidden="true"><span class="line-number">1</span><br><span class="line-number">2</span><br><span class="line-number">3</span><br><span class="line-number">4</span><br><span class="line-number">5</span><br><span class="line-number">6</span><br><span class="line-number">7</span><br><span class="line-number">8</span><br><span class="line-number">9</span><br><span class="line-number">10</span><br></div></div><h2 id="best-practices" tabindex="-1">Best Practices <a class="header-anchor" href="#best-practices" aria-label="Permalink to &quot;Best Practices&quot;">​</a></h2><h3 id="klare-bedingungen" tabindex="-1">Klare Bedingungen <a class="header-anchor" href="#klare-bedingungen" aria-label="Permalink to &quot;Klare Bedingungen&quot;">​</a></h3><div class="language-hyp vp-adaptive-theme line-numbers-mode"><button title="Copy Code" class="copy"></button><span class="lang">hyp</span><pre class="shiki shiki-themes github-light github-dark vp-code" tabindex="0"><code><span class="line"><span>// Gut</span></span>
<span class="line"><span>if (alter &gt;= 18 &amp;&amp; punktzahl &gt;= 70) {</span></span>
<span class="line"><span>    observe &quot;Zugelassen&quot;;</span></span>
<span class="line"><span>}</span></span>
<span class="line"><span></span></span>
<span class="line"><span>// Schlecht</span></span>
<span class="line"><span>if (alter &gt;= 18 &amp;&amp; punktzahl &gt;= 70 == true) {</span></span>
<span class="line"><span>    observe &quot;Zugelassen&quot;;</span></span>
<span class="line"><span>}</span></span></code></pre><div class="line-numbers-wrapper" aria-hidden="true"><span class="line-number">1</span><br><span class="line-number">2</span><br><span class="line-number">3</span><br><span class="line-number">4</span><br><span class="line-number">5</span><br><span class="line-number">6</span><br><span class="line-number">7</span><br><span class="line-number">8</span><br><span class="line-number">9</span><br></div></div><h3 id="effiziente-schleifen" tabindex="-1">Effiziente Schleifen <a class="header-anchor" href="#effiziente-schleifen" aria-label="Permalink to &quot;Effiziente Schleifen&quot;">​</a></h3><div class="language-hyp vp-adaptive-theme line-numbers-mode"><button title="Copy Code" class="copy"></button><span class="lang">hyp</span><pre class="shiki shiki-themes github-light github-dark vp-code" tabindex="0"><code><span class="line"><span>// Gut - Array-Länge einmal berechnen</span></span>
<span class="line"><span>induce laenge = ArrayLength(zahlen);</span></span>
<span class="line"><span>for (induce i = 0; i &lt; laenge; induce i = i + 1) {</span></span>
<span class="line"><span>    // Code</span></span>
<span class="line"><span>}</span></span>
<span class="line"><span></span></span>
<span class="line"><span>// Schlecht - Array-Länge bei jedem Durchlauf berechnen</span></span>
<span class="line"><span>for (induce i = 0; i &lt; ArrayLength(zahlen); induce i = i + 1) {</span></span>
<span class="line"><span>    // Code</span></span>
<span class="line"><span>}</span></span></code></pre><div class="line-numbers-wrapper" aria-hidden="true"><span class="line-number">1</span><br><span class="line-number">2</span><br><span class="line-number">3</span><br><span class="line-number">4</span><br><span class="line-number">5</span><br><span class="line-number">6</span><br><span class="line-number">7</span><br><span class="line-number">8</span><br><span class="line-number">9</span><br><span class="line-number">10</span><br></div></div><h3 id="vermeidung-von-endlosschleifen" tabindex="-1">Vermeidung von Endlosschleifen <a class="header-anchor" href="#vermeidung-von-endlosschleifen" aria-label="Permalink to &quot;Vermeidung von Endlosschleifen&quot;">​</a></h3><div class="language-hyp vp-adaptive-theme line-numbers-mode"><button title="Copy Code" class="copy"></button><span class="lang">hyp</span><pre class="shiki shiki-themes github-light github-dark vp-code" tabindex="0"><code><span class="line"><span>// Sicher - mit Break-Bedingung</span></span>
<span class="line"><span>induce zaehler = 0;</span></span>
<span class="line"><span>while (true) {</span></span>
<span class="line"><span>    induce zaehler = zaehler + 1;</span></span>
<span class="line"><span>    if (zaehler &gt; 100) {</span></span>
<span class="line"><span>        break;</span></span>
<span class="line"><span>    }</span></span>
<span class="line"><span>    // Code</span></span>
<span class="line"><span>}</span></span></code></pre><div class="line-numbers-wrapper" aria-hidden="true"><span class="line-number">1</span><br><span class="line-number">2</span><br><span class="line-number">3</span><br><span class="line-number">4</span><br><span class="line-number">5</span><br><span class="line-number">6</span><br><span class="line-number">7</span><br><span class="line-number">8</span><br><span class="line-number">9</span><br></div></div><h2 id="beispiele-fur-komplexe-kontrollstrukturen" tabindex="-1">Beispiele für komplexe Kontrollstrukturen <a class="header-anchor" href="#beispiele-fur-komplexe-kontrollstrukturen" aria-label="Permalink to &quot;Beispiele für komplexe Kontrollstrukturen&quot;">​</a></h2><h3 id="zahlenraten-spiel" tabindex="-1">Zahlenraten-Spiel <a class="header-anchor" href="#zahlenraten-spiel" aria-label="Permalink to &quot;Zahlenraten-Spiel&quot;">​</a></h3><div class="language-hyp vp-adaptive-theme line-numbers-mode"><button title="Copy Code" class="copy"></button><span class="lang">hyp</span><pre class="shiki shiki-themes github-light github-dark vp-code" tabindex="0"><code><span class="line"><span>Focus {</span></span>
<span class="line"><span>    entrance {</span></span>
<span class="line"><span>        induce zielZahl = 42;</span></span>
<span class="line"><span>        induce versuche = 0;</span></span>
<span class="line"><span>        induce maxVersuche = 10;</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        while (versuche &lt; maxVersuche) {</span></span>
<span class="line"><span>            induce versuche = versuche + 1;</span></span>
<span class="line"><span>            induce rateZahl = 25 + versuche * 2; // Vereinfachte Eingabe</span></span>
<span class="line"><span></span></span>
<span class="line"><span>            if (rateZahl == zielZahl) {</span></span>
<span class="line"><span>                observe &quot;Gewonnen! Die Zahl war &quot; + zielZahl;</span></span>
<span class="line"><span>                observe &quot;Versuche: &quot; + versuche;</span></span>
<span class="line"><span>                break;</span></span>
<span class="line"><span>            } else if (rateZahl &lt; zielZahl) {</span></span>
<span class="line"><span>                observe &quot;Zu niedrig! Versuch &quot; + versuche;</span></span>
<span class="line"><span>            } else {</span></span>
<span class="line"><span>                observe &quot;Zu hoch! Versuch &quot; + versuche;</span></span>
<span class="line"><span>            }</span></span>
<span class="line"><span>        }</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        if (versuche &gt;= maxVersuche) {</span></span>
<span class="line"><span>            observe &quot;Verloren! Die Zahl war &quot; + zielZahl;</span></span>
<span class="line"><span>        }</span></span>
<span class="line"><span>    }</span></span>
<span class="line"><span>} Relax;</span></span></code></pre><div class="line-numbers-wrapper" aria-hidden="true"><span class="line-number">1</span><br><span class="line-number">2</span><br><span class="line-number">3</span><br><span class="line-number">4</span><br><span class="line-number">5</span><br><span class="line-number">6</span><br><span class="line-number">7</span><br><span class="line-number">8</span><br><span class="line-number">9</span><br><span class="line-number">10</span><br><span class="line-number">11</span><br><span class="line-number">12</span><br><span class="line-number">13</span><br><span class="line-number">14</span><br><span class="line-number">15</span><br><span class="line-number">16</span><br><span class="line-number">17</span><br><span class="line-number">18</span><br><span class="line-number">19</span><br><span class="line-number">20</span><br><span class="line-number">21</span><br><span class="line-number">22</span><br><span class="line-number">23</span><br><span class="line-number">24</span><br><span class="line-number">25</span><br><span class="line-number">26</span><br></div></div><h3 id="array-verarbeitung-mit-bedingungen" tabindex="-1">Array-Verarbeitung mit Bedingungen <a class="header-anchor" href="#array-verarbeitung-mit-bedingungen" aria-label="Permalink to &quot;Array-Verarbeitung mit Bedingungen&quot;">​</a></h3><div class="language-hyp vp-adaptive-theme line-numbers-mode"><button title="Copy Code" class="copy"></button><span class="lang">hyp</span><pre class="shiki shiki-themes github-light github-dark vp-code" tabindex="0"><code><span class="line"><span>Focus {</span></span>
<span class="line"><span>    entrance {</span></span>
<span class="line"><span>        induce zahlen = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];</span></span>
<span class="line"><span>        induce geradeSumme = 0;</span></span>
<span class="line"><span>        induce ungeradeAnzahl = 0;</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        for (induce i = 0; i &lt; ArrayLength(zahlen); induce i = i + 1) {</span></span>
<span class="line"><span>            induce zahl = ArrayGet(zahlen, i);</span></span>
<span class="line"><span></span></span>
<span class="line"><span>            if (zahl % 2 == 0) {</span></span>
<span class="line"><span>                induce geradeSumme = geradeSumme + zahl;</span></span>
<span class="line"><span>            } else {</span></span>
<span class="line"><span>                induce ungeradeAnzahl = ungeradeAnzahl + 1;</span></span>
<span class="line"><span>            }</span></span>
<span class="line"><span>        }</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        observe &quot;Summe der geraden Zahlen: &quot; + geradeSumme;</span></span>
<span class="line"><span>        observe &quot;Anzahl der ungeraden Zahlen: &quot; + ungeradeAnzahl;</span></span>
<span class="line"><span>    }</span></span>
<span class="line"><span>} Relax;</span></span></code></pre><div class="line-numbers-wrapper" aria-hidden="true"><span class="line-number">1</span><br><span class="line-number">2</span><br><span class="line-number">3</span><br><span class="line-number">4</span><br><span class="line-number">5</span><br><span class="line-number">6</span><br><span class="line-number">7</span><br><span class="line-number">8</span><br><span class="line-number">9</span><br><span class="line-number">10</span><br><span class="line-number">11</span><br><span class="line-number">12</span><br><span class="line-number">13</span><br><span class="line-number">14</span><br><span class="line-number">15</span><br><span class="line-number">16</span><br><span class="line-number">17</span><br><span class="line-number">18</span><br><span class="line-number">19</span><br><span class="line-number">20</span><br></div></div><h2 id="nachste-schritte" tabindex="-1">Nächste Schritte <a class="header-anchor" href="#nachste-schritte" aria-label="Permalink to &quot;Nächste Schritte&quot;">​</a></h2><ul><li><a href="./functions.html">Funktionen</a> - Funktionsdefinition und -aufruf</li><li><a href="./sessions.html">Sessions</a> - Session-Management</li><li><a href="./tranceify.html">Tranceify</a> - Hypnotische Anwendungen</li><li><a href="./assertions.html">Assertions</a> - Test-Assertions</li></ul><hr><p><strong>Beherrschst du die Kontrollstrukturen? Dann lerne <a href="./functions.html">Funktionen</a> kennen!</strong> 🔧</p>`,46)])])}const d=s(l,[["render",i]]);export{h as __pageData,d as default};
