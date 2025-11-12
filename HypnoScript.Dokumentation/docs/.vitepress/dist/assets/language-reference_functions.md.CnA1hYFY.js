import{_ as s,c as a,o as e,ag as p}from"./chunks/framework.Dli2S8Ej.js";const o=JSON.parse('{"title":"Funktionen","description":"","frontmatter":{"sidebar_position":5},"headers":[],"relativePath":"language-reference/functions.md","filePath":"language-reference/functions.md","lastUpdated":1750547232000}'),l={name:"language-reference/functions.md"};function r(i,n,c,b,t,u){return e(),a("div",null,[...n[0]||(n[0]=[p(`<h1 id="funktionen" tabindex="-1">Funktionen <a class="header-anchor" href="#funktionen" aria-label="Permalink to &quot;Funktionen&quot;">​</a></h1><p>Funktionen in HypnoScript werden mit dem Schlüsselwort <code>Trance</code> definiert und ermöglichen die Modularisierung und Wiederverwendung von Code.</p><h2 id="funktionsdefinition" tabindex="-1">Funktionsdefinition <a class="header-anchor" href="#funktionsdefinition" aria-label="Permalink to &quot;Funktionsdefinition&quot;">​</a></h2><h3 id="grundlegende-syntax" tabindex="-1">Grundlegende Syntax <a class="header-anchor" href="#grundlegende-syntax" aria-label="Permalink to &quot;Grundlegende Syntax&quot;">​</a></h3><div class="language-hyp vp-adaptive-theme line-numbers-mode"><button title="Copy Code" class="copy"></button><span class="lang">hyp</span><pre class="shiki shiki-themes github-light github-dark vp-code" tabindex="0"><code><span class="line"><span>Trance funktionsName(parameter1, parameter2) {</span></span>
<span class="line"><span>    // Funktionskörper</span></span>
<span class="line"><span>    return wert; // Optional</span></span>
<span class="line"><span>}</span></span></code></pre><div class="line-numbers-wrapper" aria-hidden="true"><span class="line-number">1</span><br><span class="line-number">2</span><br><span class="line-number">3</span><br><span class="line-number">4</span><br></div></div><h3 id="einfache-funktion-ohne-parameter" tabindex="-1">Einfache Funktion ohne Parameter <a class="header-anchor" href="#einfache-funktion-ohne-parameter" aria-label="Permalink to &quot;Einfache Funktion ohne Parameter&quot;">​</a></h3><div class="language-hyp vp-adaptive-theme line-numbers-mode"><button title="Copy Code" class="copy"></button><span class="lang">hyp</span><pre class="shiki shiki-themes github-light github-dark vp-code" tabindex="0"><code><span class="line"><span>Focus {</span></span>
<span class="line"><span>    Trance begruessung() {</span></span>
<span class="line"><span>        observe &quot;Hallo, HypnoScript!&quot;;</span></span>
<span class="line"><span>    }</span></span>
<span class="line"><span></span></span>
<span class="line"><span>    entrance {</span></span>
<span class="line"><span>        begruessung();</span></span>
<span class="line"><span>    }</span></span>
<span class="line"><span>} Relax;</span></span></code></pre><div class="line-numbers-wrapper" aria-hidden="true"><span class="line-number">1</span><br><span class="line-number">2</span><br><span class="line-number">3</span><br><span class="line-number">4</span><br><span class="line-number">5</span><br><span class="line-number">6</span><br><span class="line-number">7</span><br><span class="line-number">8</span><br><span class="line-number">9</span><br></div></div><h3 id="funktion-mit-parametern" tabindex="-1">Funktion mit Parametern <a class="header-anchor" href="#funktion-mit-parametern" aria-label="Permalink to &quot;Funktion mit Parametern&quot;">​</a></h3><div class="language-hyp vp-adaptive-theme line-numbers-mode"><button title="Copy Code" class="copy"></button><span class="lang">hyp</span><pre class="shiki shiki-themes github-light github-dark vp-code" tabindex="0"><code><span class="line"><span>Focus {</span></span>
<span class="line"><span>    Trance begruesse(name) {</span></span>
<span class="line"><span>        observe &quot;Hallo, &quot; + name + &quot;!&quot;;</span></span>
<span class="line"><span>    }</span></span>
<span class="line"><span></span></span>
<span class="line"><span>    entrance {</span></span>
<span class="line"><span>        begruesse(&quot;Max&quot;);</span></span>
<span class="line"><span>        begruesse(&quot;Anna&quot;);</span></span>
<span class="line"><span>    }</span></span>
<span class="line"><span>} Relax;</span></span></code></pre><div class="line-numbers-wrapper" aria-hidden="true"><span class="line-number">1</span><br><span class="line-number">2</span><br><span class="line-number">3</span><br><span class="line-number">4</span><br><span class="line-number">5</span><br><span class="line-number">6</span><br><span class="line-number">7</span><br><span class="line-number">8</span><br><span class="line-number">9</span><br><span class="line-number">10</span><br></div></div><h3 id="funktion-mit-ruckgabewert" tabindex="-1">Funktion mit Rückgabewert <a class="header-anchor" href="#funktion-mit-ruckgabewert" aria-label="Permalink to &quot;Funktion mit Rückgabewert&quot;">​</a></h3><div class="language-hyp vp-adaptive-theme line-numbers-mode"><button title="Copy Code" class="copy"></button><span class="lang">hyp</span><pre class="shiki shiki-themes github-light github-dark vp-code" tabindex="0"><code><span class="line"><span>Focus {</span></span>
<span class="line"><span>    Trance addiere(a, b) {</span></span>
<span class="line"><span>        return a + b;</span></span>
<span class="line"><span>    }</span></span>
<span class="line"><span></span></span>
<span class="line"><span>    Trance istGerade(zahl) {</span></span>
<span class="line"><span>        return zahl % 2 == 0;</span></span>
<span class="line"><span>    }</span></span>
<span class="line"><span></span></span>
<span class="line"><span>    entrance {</span></span>
<span class="line"><span>        induce summe = addiere(5, 3);</span></span>
<span class="line"><span>        observe &quot;5 + 3 = &quot; + summe;</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        induce check = istGerade(42);</span></span>
<span class="line"><span>        observe &quot;42 ist gerade: &quot; + check;</span></span>
<span class="line"><span>    }</span></span>
<span class="line"><span>} Relax;</span></span></code></pre><div class="line-numbers-wrapper" aria-hidden="true"><span class="line-number">1</span><br><span class="line-number">2</span><br><span class="line-number">3</span><br><span class="line-number">4</span><br><span class="line-number">5</span><br><span class="line-number">6</span><br><span class="line-number">7</span><br><span class="line-number">8</span><br><span class="line-number">9</span><br><span class="line-number">10</span><br><span class="line-number">11</span><br><span class="line-number">12</span><br><span class="line-number">13</span><br><span class="line-number">14</span><br><span class="line-number">15</span><br><span class="line-number">16</span><br><span class="line-number">17</span><br></div></div><h2 id="parameter" tabindex="-1">Parameter <a class="header-anchor" href="#parameter" aria-label="Permalink to &quot;Parameter&quot;">​</a></h2><h3 id="mehrere-parameter" tabindex="-1">Mehrere Parameter <a class="header-anchor" href="#mehrere-parameter" aria-label="Permalink to &quot;Mehrere Parameter&quot;">​</a></h3><div class="language-hyp vp-adaptive-theme line-numbers-mode"><button title="Copy Code" class="copy"></button><span class="lang">hyp</span><pre class="shiki shiki-themes github-light github-dark vp-code" tabindex="0"><code><span class="line"><span>Focus {</span></span>
<span class="line"><span>    Trance rechteckFlaeche(breite, hoehe) {</span></span>
<span class="line"><span>        return breite * hoehe;</span></span>
<span class="line"><span>    }</span></span>
<span class="line"><span></span></span>
<span class="line"><span>    Trance personInfo(name, alter, stadt) {</span></span>
<span class="line"><span>        return &quot;Name: &quot; + name + &quot;, Alter: &quot; + alter + &quot;, Stadt: &quot; + stadt;</span></span>
<span class="line"><span>    }</span></span>
<span class="line"><span></span></span>
<span class="line"><span>    entrance {</span></span>
<span class="line"><span>        induce flaeche = rechteckFlaeche(10, 5);</span></span>
<span class="line"><span>        observe &quot;Fläche: &quot; + flaeche;</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        induce info = personInfo(&quot;Max&quot;, 30, &quot;Berlin&quot;);</span></span>
<span class="line"><span>        observe info;</span></span>
<span class="line"><span>    }</span></span>
<span class="line"><span>} Relax;</span></span></code></pre><div class="line-numbers-wrapper" aria-hidden="true"><span class="line-number">1</span><br><span class="line-number">2</span><br><span class="line-number">3</span><br><span class="line-number">4</span><br><span class="line-number">5</span><br><span class="line-number">6</span><br><span class="line-number">7</span><br><span class="line-number">8</span><br><span class="line-number">9</span><br><span class="line-number">10</span><br><span class="line-number">11</span><br><span class="line-number">12</span><br><span class="line-number">13</span><br><span class="line-number">14</span><br><span class="line-number">15</span><br><span class="line-number">16</span><br><span class="line-number">17</span><br></div></div><h3 id="parameter-mit-standardwerten" tabindex="-1">Parameter mit Standardwerten <a class="header-anchor" href="#parameter-mit-standardwerten" aria-label="Permalink to &quot;Parameter mit Standardwerten&quot;">​</a></h3><div class="language-hyp vp-adaptive-theme line-numbers-mode"><button title="Copy Code" class="copy"></button><span class="lang">hyp</span><pre class="shiki shiki-themes github-light github-dark vp-code" tabindex="0"><code><span class="line"><span>Focus {</span></span>
<span class="line"><span>    Trance begruesse(name, titel = &quot;Herr/Frau&quot;) {</span></span>
<span class="line"><span>        observe titel + &quot; &quot; + name + &quot;, willkommen!&quot;;</span></span>
<span class="line"><span>    }</span></span>
<span class="line"><span></span></span>
<span class="line"><span>    entrance {</span></span>
<span class="line"><span>        begruesse(&quot;Mustermann&quot;); // Verwendet Standardtitel</span></span>
<span class="line"><span>        begruesse(&quot;Schmidt&quot;, &quot;Dr.&quot;); // Überschreibt Standardtitel</span></span>
<span class="line"><span>    }</span></span>
<span class="line"><span>} Relax;</span></span></code></pre><div class="line-numbers-wrapper" aria-hidden="true"><span class="line-number">1</span><br><span class="line-number">2</span><br><span class="line-number">3</span><br><span class="line-number">4</span><br><span class="line-number">5</span><br><span class="line-number">6</span><br><span class="line-number">7</span><br><span class="line-number">8</span><br><span class="line-number">9</span><br><span class="line-number">10</span><br></div></div><h2 id="rekursive-funktionen" tabindex="-1">Rekursive Funktionen <a class="header-anchor" href="#rekursive-funktionen" aria-label="Permalink to &quot;Rekursive Funktionen&quot;">​</a></h2><div class="language-hyp vp-adaptive-theme line-numbers-mode"><button title="Copy Code" class="copy"></button><span class="lang">hyp</span><pre class="shiki shiki-themes github-light github-dark vp-code" tabindex="0"><code><span class="line"><span>Focus {</span></span>
<span class="line"><span>    Trance fakultaet(n) {</span></span>
<span class="line"><span>        if (n &lt;= 1) {</span></span>
<span class="line"><span>            return 1;</span></span>
<span class="line"><span>        } else {</span></span>
<span class="line"><span>            return n * fakultaet(n - 1);</span></span>
<span class="line"><span>        }</span></span>
<span class="line"><span>    }</span></span>
<span class="line"><span></span></span>
<span class="line"><span>    Trance fibonacci(n) {</span></span>
<span class="line"><span>        if (n &lt;= 1) {</span></span>
<span class="line"><span>            return n;</span></span>
<span class="line"><span>        } else {</span></span>
<span class="line"><span>            return fibonacci(n - 1) + fibonacci(n - 2);</span></span>
<span class="line"><span>        }</span></span>
<span class="line"><span>    }</span></span>
<span class="line"><span></span></span>
<span class="line"><span>    entrance {</span></span>
<span class="line"><span>        induce fact5 = fakultaet(5);</span></span>
<span class="line"><span>        observe &quot;5! = &quot; + fact5;</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        induce fib10 = fibonacci(10);</span></span>
<span class="line"><span>        observe &quot;Fibonacci(10) = &quot; + fib10;</span></span>
<span class="line"><span>    }</span></span>
<span class="line"><span>} Relax;</span></span></code></pre><div class="line-numbers-wrapper" aria-hidden="true"><span class="line-number">1</span><br><span class="line-number">2</span><br><span class="line-number">3</span><br><span class="line-number">4</span><br><span class="line-number">5</span><br><span class="line-number">6</span><br><span class="line-number">7</span><br><span class="line-number">8</span><br><span class="line-number">9</span><br><span class="line-number">10</span><br><span class="line-number">11</span><br><span class="line-number">12</span><br><span class="line-number">13</span><br><span class="line-number">14</span><br><span class="line-number">15</span><br><span class="line-number">16</span><br><span class="line-number">17</span><br><span class="line-number">18</span><br><span class="line-number">19</span><br><span class="line-number">20</span><br><span class="line-number">21</span><br><span class="line-number">22</span><br><span class="line-number">23</span><br><span class="line-number">24</span><br><span class="line-number">25</span><br></div></div><h2 id="funktionen-mit-arrays" tabindex="-1">Funktionen mit Arrays <a class="header-anchor" href="#funktionen-mit-arrays" aria-label="Permalink to &quot;Funktionen mit Arrays&quot;">​</a></h2><div class="language-hyp vp-adaptive-theme line-numbers-mode"><button title="Copy Code" class="copy"></button><span class="lang">hyp</span><pre class="shiki shiki-themes github-light github-dark vp-code" tabindex="0"><code><span class="line"><span>Focus {</span></span>
<span class="line"><span>    Trance arraySumme(zahlen) {</span></span>
<span class="line"><span>        induce summe = 0;</span></span>
<span class="line"><span>        for (induce i = 0; i &lt; ArrayLength(zahlen); induce i = i + 1) {</span></span>
<span class="line"><span>            induce summe = summe + ArrayGet(zahlen, i);</span></span>
<span class="line"><span>        }</span></span>
<span class="line"><span>        return summe;</span></span>
<span class="line"><span>    }</span></span>
<span class="line"><span></span></span>
<span class="line"><span>    Trance findeMaximum(zahlen) {</span></span>
<span class="line"><span>        if (ArrayLength(zahlen) == 0) {</span></span>
<span class="line"><span>            return null;</span></span>
<span class="line"><span>        }</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        induce max = ArrayGet(zahlen, 0);</span></span>
<span class="line"><span>        for (induce i = 1; i &lt; ArrayLength(zahlen); induce i = i + 1) {</span></span>
<span class="line"><span>            induce wert = ArrayGet(zahlen, i);</span></span>
<span class="line"><span>            if (wert &gt; max) {</span></span>
<span class="line"><span>                induce max = wert;</span></span>
<span class="line"><span>            }</span></span>
<span class="line"><span>        }</span></span>
<span class="line"><span>        return max;</span></span>
<span class="line"><span>    }</span></span>
<span class="line"><span></span></span>
<span class="line"><span>    Trance filterGerade(zahlen) {</span></span>
<span class="line"><span>        induce ergebnis = [];</span></span>
<span class="line"><span>        for (induce i = 0; i &lt; ArrayLength(zahlen); induce i = i + 1) {</span></span>
<span class="line"><span>            induce zahl = ArrayGet(zahlen, i);</span></span>
<span class="line"><span>            if (zahl % 2 == 0) {</span></span>
<span class="line"><span>                // Array erweitern (vereinfacht)</span></span>
<span class="line"><span>                observe &quot;Gerade Zahl gefunden: &quot; + zahl;</span></span>
<span class="line"><span>            }</span></span>
<span class="line"><span>        }</span></span>
<span class="line"><span>        return ergebnis;</span></span>
<span class="line"><span>    }</span></span>
<span class="line"><span></span></span>
<span class="line"><span>    entrance {</span></span>
<span class="line"><span>        induce testZahlen = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        induce summe = arraySumme(testZahlen);</span></span>
<span class="line"><span>        observe &quot;Summe: &quot; + summe;</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        induce max = findeMaximum(testZahlen);</span></span>
<span class="line"><span>        observe &quot;Maximum: &quot; + max;</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        filterGerade(testZahlen);</span></span>
<span class="line"><span>    }</span></span>
<span class="line"><span>} Relax;</span></span></code></pre><div class="line-numbers-wrapper" aria-hidden="true"><span class="line-number">1</span><br><span class="line-number">2</span><br><span class="line-number">3</span><br><span class="line-number">4</span><br><span class="line-number">5</span><br><span class="line-number">6</span><br><span class="line-number">7</span><br><span class="line-number">8</span><br><span class="line-number">9</span><br><span class="line-number">10</span><br><span class="line-number">11</span><br><span class="line-number">12</span><br><span class="line-number">13</span><br><span class="line-number">14</span><br><span class="line-number">15</span><br><span class="line-number">16</span><br><span class="line-number">17</span><br><span class="line-number">18</span><br><span class="line-number">19</span><br><span class="line-number">20</span><br><span class="line-number">21</span><br><span class="line-number">22</span><br><span class="line-number">23</span><br><span class="line-number">24</span><br><span class="line-number">25</span><br><span class="line-number">26</span><br><span class="line-number">27</span><br><span class="line-number">28</span><br><span class="line-number">29</span><br><span class="line-number">30</span><br><span class="line-number">31</span><br><span class="line-number">32</span><br><span class="line-number">33</span><br><span class="line-number">34</span><br><span class="line-number">35</span><br><span class="line-number">36</span><br><span class="line-number">37</span><br><span class="line-number">38</span><br><span class="line-number">39</span><br><span class="line-number">40</span><br><span class="line-number">41</span><br><span class="line-number">42</span><br><span class="line-number">43</span><br><span class="line-number">44</span><br><span class="line-number">45</span><br><span class="line-number">46</span><br><span class="line-number">47</span><br><span class="line-number">48</span><br></div></div><h2 id="funktionen-mit-records" tabindex="-1">Funktionen mit Records <a class="header-anchor" href="#funktionen-mit-records" aria-label="Permalink to &quot;Funktionen mit Records&quot;">​</a></h2><div class="language-hyp vp-adaptive-theme line-numbers-mode"><button title="Copy Code" class="copy"></button><span class="lang">hyp</span><pre class="shiki shiki-themes github-light github-dark vp-code" tabindex="0"><code><span class="line"><span>Focus {</span></span>
<span class="line"><span>    Trance erstellePerson(name, alter, stadt) {</span></span>
<span class="line"><span>        return {</span></span>
<span class="line"><span>            name: name,</span></span>
<span class="line"><span>            alter: alter,</span></span>
<span class="line"><span>            stadt: stadt,</span></span>
<span class="line"><span>            volljaehrig: alter &gt;= 18</span></span>
<span class="line"><span>        };</span></span>
<span class="line"><span>    }</span></span>
<span class="line"><span></span></span>
<span class="line"><span>    Trance personInfo(person) {</span></span>
<span class="line"><span>        return person.name + &quot; (&quot; + person.alter + &quot;) aus &quot; + person.stadt;</span></span>
<span class="line"><span>    }</span></span>
<span class="line"><span></span></span>
<span class="line"><span>    Trance istVolljaehrig(person) {</span></span>
<span class="line"><span>        return person.volljaehrig;</span></span>
<span class="line"><span>    }</span></span>
<span class="line"><span></span></span>
<span class="line"><span>    entrance {</span></span>
<span class="line"><span>        induce person1 = erstellePerson(&quot;Max&quot;, 25, &quot;Berlin&quot;);</span></span>
<span class="line"><span>        induce person2 = erstellePerson(&quot;Anna&quot;, 16, &quot;Hamburg&quot;);</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        observe personInfo(person1);</span></span>
<span class="line"><span>        observe personInfo(person2);</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        observe &quot;Max ist volljährig: &quot; + istVolljaehrig(person1);</span></span>
<span class="line"><span>        observe &quot;Anna ist volljährig: &quot; + istVolljaehrig(person2);</span></span>
<span class="line"><span>    }</span></span>
<span class="line"><span>} Relax;</span></span></code></pre><div class="line-numbers-wrapper" aria-hidden="true"><span class="line-number">1</span><br><span class="line-number">2</span><br><span class="line-number">3</span><br><span class="line-number">4</span><br><span class="line-number">5</span><br><span class="line-number">6</span><br><span class="line-number">7</span><br><span class="line-number">8</span><br><span class="line-number">9</span><br><span class="line-number">10</span><br><span class="line-number">11</span><br><span class="line-number">12</span><br><span class="line-number">13</span><br><span class="line-number">14</span><br><span class="line-number">15</span><br><span class="line-number">16</span><br><span class="line-number">17</span><br><span class="line-number">18</span><br><span class="line-number">19</span><br><span class="line-number">20</span><br><span class="line-number">21</span><br><span class="line-number">22</span><br><span class="line-number">23</span><br><span class="line-number">24</span><br><span class="line-number">25</span><br><span class="line-number">26</span><br><span class="line-number">27</span><br><span class="line-number">28</span><br><span class="line-number">29</span><br></div></div><h2 id="hilfsfunktionen" tabindex="-1">Hilfsfunktionen <a class="header-anchor" href="#hilfsfunktionen" aria-label="Permalink to &quot;Hilfsfunktionen&quot;">​</a></h2><div class="language-hyp vp-adaptive-theme line-numbers-mode"><button title="Copy Code" class="copy"></button><span class="lang">hyp</span><pre class="shiki shiki-themes github-light github-dark vp-code" tabindex="0"><code><span class="line"><span>Focus {</span></span>
<span class="line"><span>    Trance validiereAlter(alter) {</span></span>
<span class="line"><span>        return alter &gt;= 0 &amp;&amp; alter &lt;= 150;</span></span>
<span class="line"><span>    }</span></span>
<span class="line"><span></span></span>
<span class="line"><span>    Trance validiereEmail(email) {</span></span>
<span class="line"><span>        // Einfache E-Mail-Validierung</span></span>
<span class="line"><span>        return Length(email) &gt; 0 &amp;&amp; email != null;</span></span>
<span class="line"><span>    }</span></span>
<span class="line"><span></span></span>
<span class="line"><span>    Trance berechneBMI(gewicht, groesse) {</span></span>
<span class="line"><span>        if (groesse &lt;= 0) {</span></span>
<span class="line"><span>            return null;</span></span>
<span class="line"><span>        }</span></span>
<span class="line"><span>        return gewicht / (groesse * groesse);</span></span>
<span class="line"><span>    }</span></span>
<span class="line"><span></span></span>
<span class="line"><span>    Trance bmiKategorie(bmi) {</span></span>
<span class="line"><span>        if (bmi == null) {</span></span>
<span class="line"><span>            return &quot;Ungültig&quot;;</span></span>
<span class="line"><span>        } else if (bmi &lt; 18.5) {</span></span>
<span class="line"><span>            return &quot;Untergewicht&quot;;</span></span>
<span class="line"><span>        } else if (bmi &lt; 25) {</span></span>
<span class="line"><span>            return &quot;Normalgewicht&quot;;</span></span>
<span class="line"><span>        } else if (bmi &lt; 30) {</span></span>
<span class="line"><span>            return &quot;Übergewicht&quot;;</span></span>
<span class="line"><span>        } else {</span></span>
<span class="line"><span>            return &quot;Adipositas&quot;;</span></span>
<span class="line"><span>        }</span></span>
<span class="line"><span>    }</span></span>
<span class="line"><span></span></span>
<span class="line"><span>    entrance {</span></span>
<span class="line"><span>        induce alter = 25;</span></span>
<span class="line"><span>        induce email = &quot;test@example.com&quot;;</span></span>
<span class="line"><span>        induce gewicht = 70;</span></span>
<span class="line"><span>        induce groesse = 1.75;</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        if (validiereAlter(alter)) {</span></span>
<span class="line"><span>            observe &quot;Alter ist gültig&quot;;</span></span>
<span class="line"><span>        }</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        if (validiereEmail(email)) {</span></span>
<span class="line"><span>            observe &quot;E-Mail ist gültig&quot;;</span></span>
<span class="line"><span>        }</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        induce bmi = berechneBMI(gewicht, groesse);</span></span>
<span class="line"><span>        induce kategorie = bmiKategorie(bmi);</span></span>
<span class="line"><span>        observe &quot;BMI: &quot; + bmi + &quot; (&quot; + kategorie + &quot;)&quot;;</span></span>
<span class="line"><span>    }</span></span>
<span class="line"><span>} Relax;</span></span></code></pre><div class="line-numbers-wrapper" aria-hidden="true"><span class="line-number">1</span><br><span class="line-number">2</span><br><span class="line-number">3</span><br><span class="line-number">4</span><br><span class="line-number">5</span><br><span class="line-number">6</span><br><span class="line-number">7</span><br><span class="line-number">8</span><br><span class="line-number">9</span><br><span class="line-number">10</span><br><span class="line-number">11</span><br><span class="line-number">12</span><br><span class="line-number">13</span><br><span class="line-number">14</span><br><span class="line-number">15</span><br><span class="line-number">16</span><br><span class="line-number">17</span><br><span class="line-number">18</span><br><span class="line-number">19</span><br><span class="line-number">20</span><br><span class="line-number">21</span><br><span class="line-number">22</span><br><span class="line-number">23</span><br><span class="line-number">24</span><br><span class="line-number">25</span><br><span class="line-number">26</span><br><span class="line-number">27</span><br><span class="line-number">28</span><br><span class="line-number">29</span><br><span class="line-number">30</span><br><span class="line-number">31</span><br><span class="line-number">32</span><br><span class="line-number">33</span><br><span class="line-number">34</span><br><span class="line-number">35</span><br><span class="line-number">36</span><br><span class="line-number">37</span><br><span class="line-number">38</span><br><span class="line-number">39</span><br><span class="line-number">40</span><br><span class="line-number">41</span><br><span class="line-number">42</span><br><span class="line-number">43</span><br><span class="line-number">44</span><br><span class="line-number">45</span><br><span class="line-number">46</span><br><span class="line-number">47</span><br><span class="line-number">48</span><br><span class="line-number">49</span><br><span class="line-number">50</span><br></div></div><h2 id="mathematische-funktionen" tabindex="-1">Mathematische Funktionen <a class="header-anchor" href="#mathematische-funktionen" aria-label="Permalink to &quot;Mathematische Funktionen&quot;">​</a></h2><div class="language-hyp vp-adaptive-theme line-numbers-mode"><button title="Copy Code" class="copy"></button><span class="lang">hyp</span><pre class="shiki shiki-themes github-light github-dark vp-code" tabindex="0"><code><span class="line"><span>Focus {</span></span>
<span class="line"><span>    Trance potenz(basis, exponent) {</span></span>
<span class="line"><span>        if (exponent == 0) {</span></span>
<span class="line"><span>            return 1;</span></span>
<span class="line"><span>        }</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        induce ergebnis = 1;</span></span>
<span class="line"><span>        for (induce i = 1; i &lt;= exponent; induce i = i + 1) {</span></span>
<span class="line"><span>            induce ergebnis = ergebnis * basis;</span></span>
<span class="line"><span>        }</span></span>
<span class="line"><span>        return ergebnis;</span></span>
<span class="line"><span>    }</span></span>
<span class="line"><span></span></span>
<span class="line"><span>    Trance istPrimzahl(zahl) {</span></span>
<span class="line"><span>        if (zahl &lt; 2) {</span></span>
<span class="line"><span>            return false;</span></span>
<span class="line"><span>        }</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        for (induce i = 2; i * i &lt;= zahl; induce i = i + 1) {</span></span>
<span class="line"><span>            if (zahl % i == 0) {</span></span>
<span class="line"><span>                return false;</span></span>
<span class="line"><span>            }</span></span>
<span class="line"><span>        }</span></span>
<span class="line"><span>        return true;</span></span>
<span class="line"><span>    }</span></span>
<span class="line"><span></span></span>
<span class="line"><span>    Trance ggT(a, b) {</span></span>
<span class="line"><span>        while (b != 0) {</span></span>
<span class="line"><span>            induce temp = b;</span></span>
<span class="line"><span>            induce b = a % b;</span></span>
<span class="line"><span>            induce a = temp;</span></span>
<span class="line"><span>        }</span></span>
<span class="line"><span>        return a;</span></span>
<span class="line"><span>    }</span></span>
<span class="line"><span></span></span>
<span class="line"><span>    entrance {</span></span>
<span class="line"><span>        observe &quot;2^10 = &quot; + potenz(2, 10);</span></span>
<span class="line"><span>        observe &quot;17 ist Primzahl: &quot; + istPrimzahl(17);</span></span>
<span class="line"><span>        observe &quot;GGT von 48 und 18: &quot; + ggT(48, 18);</span></span>
<span class="line"><span>    }</span></span>
<span class="line"><span>} Relax;</span></span></code></pre><div class="line-numbers-wrapper" aria-hidden="true"><span class="line-number">1</span><br><span class="line-number">2</span><br><span class="line-number">3</span><br><span class="line-number">4</span><br><span class="line-number">5</span><br><span class="line-number">6</span><br><span class="line-number">7</span><br><span class="line-number">8</span><br><span class="line-number">9</span><br><span class="line-number">10</span><br><span class="line-number">11</span><br><span class="line-number">12</span><br><span class="line-number">13</span><br><span class="line-number">14</span><br><span class="line-number">15</span><br><span class="line-number">16</span><br><span class="line-number">17</span><br><span class="line-number">18</span><br><span class="line-number">19</span><br><span class="line-number">20</span><br><span class="line-number">21</span><br><span class="line-number">22</span><br><span class="line-number">23</span><br><span class="line-number">24</span><br><span class="line-number">25</span><br><span class="line-number">26</span><br><span class="line-number">27</span><br><span class="line-number">28</span><br><span class="line-number">29</span><br><span class="line-number">30</span><br><span class="line-number">31</span><br><span class="line-number">32</span><br><span class="line-number">33</span><br><span class="line-number">34</span><br><span class="line-number">35</span><br><span class="line-number">36</span><br><span class="line-number">37</span><br><span class="line-number">38</span><br><span class="line-number">39</span><br><span class="line-number">40</span><br><span class="line-number">41</span><br></div></div><h2 id="best-practices" tabindex="-1">Best Practices <a class="header-anchor" href="#best-practices" aria-label="Permalink to &quot;Best Practices&quot;">​</a></h2><h3 id="funktionen-benennen" tabindex="-1">Funktionen benennen <a class="header-anchor" href="#funktionen-benennen" aria-label="Permalink to &quot;Funktionen benennen&quot;">​</a></h3><div class="language-hyp vp-adaptive-theme line-numbers-mode"><button title="Copy Code" class="copy"></button><span class="lang">hyp</span><pre class="shiki shiki-themes github-light github-dark vp-code" tabindex="0"><code><span class="line"><span>// Gut - beschreibende Namen</span></span>
<span class="line"><span>Trance berechneDurchschnitt(zahlen) { ... }</span></span>
<span class="line"><span>Trance istGueltigeEmail(email) { ... }</span></span>
<span class="line"><span>Trance formatiereDatum(datum) { ... }</span></span>
<span class="line"><span></span></span>
<span class="line"><span>// Schlecht - unklare Namen</span></span>
<span class="line"><span>Trance calc(arr) { ... }</span></span>
<span class="line"><span>Trance check(str) { ... }</span></span>
<span class="line"><span>Trance format(d) { ... }</span></span></code></pre><div class="line-numbers-wrapper" aria-hidden="true"><span class="line-number">1</span><br><span class="line-number">2</span><br><span class="line-number">3</span><br><span class="line-number">4</span><br><span class="line-number">5</span><br><span class="line-number">6</span><br><span class="line-number">7</span><br><span class="line-number">8</span><br><span class="line-number">9</span><br></div></div><h3 id="einzelverantwortlichkeit" tabindex="-1">Einzelverantwortlichkeit <a class="header-anchor" href="#einzelverantwortlichkeit" aria-label="Permalink to &quot;Einzelverantwortlichkeit&quot;">​</a></h3><div class="language-hyp vp-adaptive-theme line-numbers-mode"><button title="Copy Code" class="copy"></button><span class="lang">hyp</span><pre class="shiki shiki-themes github-light github-dark vp-code" tabindex="0"><code><span class="line"><span>// Gut - eine Funktion, eine Aufgabe</span></span>
<span class="line"><span>Trance validiereAlter(alter) {</span></span>
<span class="line"><span>    return alter &gt;= 0 &amp;&amp; alter &lt;= 150;</span></span>
<span class="line"><span>}</span></span>
<span class="line"><span></span></span>
<span class="line"><span>Trance berechneAltersgruppe(alter) {</span></span>
<span class="line"><span>    if (alter &lt; 18) return &quot;Jugendlich&quot;;</span></span>
<span class="line"><span>    if (alter &lt; 65) return &quot;Erwachsen&quot;;</span></span>
<span class="line"><span>    return &quot;Senior&quot;;</span></span>
<span class="line"><span>}</span></span>
<span class="line"><span></span></span>
<span class="line"><span>// Schlecht - zu viele Aufgaben in einer Funktion</span></span>
<span class="line"><span>Trance verarbeitePerson(alter, name, email) {</span></span>
<span class="line"><span>    // Validierung, Berechnung, Formatierung alles in einer Funktion</span></span>
<span class="line"><span>}</span></span></code></pre><div class="line-numbers-wrapper" aria-hidden="true"><span class="line-number">1</span><br><span class="line-number">2</span><br><span class="line-number">3</span><br><span class="line-number">4</span><br><span class="line-number">5</span><br><span class="line-number">6</span><br><span class="line-number">7</span><br><span class="line-number">8</span><br><span class="line-number">9</span><br><span class="line-number">10</span><br><span class="line-number">11</span><br><span class="line-number">12</span><br><span class="line-number">13</span><br><span class="line-number">14</span><br><span class="line-number">15</span><br></div></div><h3 id="fehlerbehandlung" tabindex="-1">Fehlerbehandlung <a class="header-anchor" href="#fehlerbehandlung" aria-label="Permalink to &quot;Fehlerbehandlung&quot;">​</a></h3><div class="language-hyp vp-adaptive-theme line-numbers-mode"><button title="Copy Code" class="copy"></button><span class="lang">hyp</span><pre class="shiki shiki-themes github-light github-dark vp-code" tabindex="0"><code><span class="line"><span>Focus {</span></span>
<span class="line"><span>    Trance sichereDivision(a, b) {</span></span>
<span class="line"><span>        if (b == 0) {</span></span>
<span class="line"><span>            observe &quot;Fehler: Division durch Null!&quot;;</span></span>
<span class="line"><span>            return null;</span></span>
<span class="line"><span>        }</span></span>
<span class="line"><span>        return a / b;</span></span>
<span class="line"><span>    }</span></span>
<span class="line"><span></span></span>
<span class="line"><span>    Trance arrayElementSicher(arr, index) {</span></span>
<span class="line"><span>        if (index &lt; 0 || index &gt;= ArrayLength(arr)) {</span></span>
<span class="line"><span>            observe &quot;Fehler: Index außerhalb des Bereichs!&quot;;</span></span>
<span class="line"><span>            return null;</span></span>
<span class="line"><span>        }</span></span>
<span class="line"><span>        return ArrayGet(arr, index);</span></span>
<span class="line"><span>    }</span></span>
<span class="line"><span></span></span>
<span class="line"><span>    entrance {</span></span>
<span class="line"><span>        induce ergebnis1 = sichereDivision(10, 0);</span></span>
<span class="line"><span>        induce ergebnis2 = sichereDivision(10, 2);</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        induce zahlen = [1, 2, 3];</span></span>
<span class="line"><span>        induce element1 = arrayElementSicher(zahlen, 5);</span></span>
<span class="line"><span>        induce element2 = arrayElementSicher(zahlen, 1);</span></span>
<span class="line"><span>    }</span></span>
<span class="line"><span>} Relax;</span></span></code></pre><div class="line-numbers-wrapper" aria-hidden="true"><span class="line-number">1</span><br><span class="line-number">2</span><br><span class="line-number">3</span><br><span class="line-number">4</span><br><span class="line-number">5</span><br><span class="line-number">6</span><br><span class="line-number">7</span><br><span class="line-number">8</span><br><span class="line-number">9</span><br><span class="line-number">10</span><br><span class="line-number">11</span><br><span class="line-number">12</span><br><span class="line-number">13</span><br><span class="line-number">14</span><br><span class="line-number">15</span><br><span class="line-number">16</span><br><span class="line-number">17</span><br><span class="line-number">18</span><br><span class="line-number">19</span><br><span class="line-number">20</span><br><span class="line-number">21</span><br><span class="line-number">22</span><br><span class="line-number">23</span><br><span class="line-number">24</span><br><span class="line-number">25</span><br><span class="line-number">26</span><br></div></div><h2 id="nachste-schritte" tabindex="-1">Nächste Schritte <a class="header-anchor" href="#nachste-schritte" aria-label="Permalink to &quot;Nächste Schritte&quot;">​</a></h2><ul><li><a href="./sessions.html">Sessions</a> - Session-Management</li><li><a href="./tranceify.html">Tranceify</a> - Hypnotische Anwendungen</li><li><a href="./arrays.html">Arrays</a> - Array-Operationen</li><li><a href="./records.html">Records</a> - Objekt-Programmierung</li></ul><hr><p><strong>Beherrschst du Funktionen? Dann lerne <a href="./sessions.html">Sessions</a> kennen!</strong> 🧠</p>`,37)])])}const d=s(l,[["render",r]]);export{o as __pageData,d as default};
