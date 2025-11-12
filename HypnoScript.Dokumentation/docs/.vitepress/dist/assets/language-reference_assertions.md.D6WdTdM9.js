import{_ as n,c as a,o as e,ag as p}from"./chunks/framework.Dli2S8Ej.js";const m=JSON.parse('{"title":"Assertions","description":"","frontmatter":{"title":"Assertions"},"headers":[],"relativePath":"language-reference/assertions.md","filePath":"language-reference/assertions.md","lastUpdated":1750802436000}'),l={name:"language-reference/assertions.md"};function r(i,s,t,c,u,b){return e(),a("div",null,[...s[0]||(s[0]=[p(`<h1 id="assertions" tabindex="-1">Assertions <a class="header-anchor" href="#assertions" aria-label="Permalink to &quot;Assertions&quot;">​</a></h1><p>Assertions sind mächtige Werkzeuge in HypnoScript, um Bedingungen zu überprüfen und Fehler frühzeitig zu erkennen.</p><h2 id="ubersicht" tabindex="-1">Übersicht <a class="header-anchor" href="#ubersicht" aria-label="Permalink to &quot;Übersicht&quot;">​</a></h2><p>Assertions ermöglichen es Ihnen, Annahmen über den Zustand Ihres Programms zu formulieren und automatisch zu überprüfen. Sie sind besonders nützlich für Debugging, Testing und die Validierung von Eingabedaten.</p><h2 id="grundlegende-syntax" tabindex="-1">Grundlegende Syntax <a class="header-anchor" href="#grundlegende-syntax" aria-label="Permalink to &quot;Grundlegende Syntax&quot;">​</a></h2><h3 id="einfache-assertion" tabindex="-1">Einfache Assertion <a class="header-anchor" href="#einfache-assertion" aria-label="Permalink to &quot;Einfache Assertion&quot;">​</a></h3><div class="language-hyp vp-adaptive-theme line-numbers-mode"><button title="Copy Code" class="copy"></button><span class="lang">hyp</span><pre class="shiki shiki-themes github-light github-dark vp-code" tabindex="0"><code><span class="line"><span>assert condition &quot;Optional message&quot;;</span></span></code></pre><div class="line-numbers-wrapper" aria-hidden="true"><span class="line-number">1</span><br></div></div><h3 id="assertion-ohne-nachricht" tabindex="-1">Assertion ohne Nachricht <a class="header-anchor" href="#assertion-ohne-nachricht" aria-label="Permalink to &quot;Assertion ohne Nachricht&quot;">​</a></h3><div class="language-hyp vp-adaptive-theme line-numbers-mode"><button title="Copy Code" class="copy"></button><span class="lang">hyp</span><pre class="shiki shiki-themes github-light github-dark vp-code" tabindex="0"><code><span class="line"><span>assert condition;</span></span></code></pre><div class="line-numbers-wrapper" aria-hidden="true"><span class="line-number">1</span><br></div></div><h2 id="grundlegende-assertions" tabindex="-1">Grundlegende Assertions <a class="header-anchor" href="#grundlegende-assertions" aria-label="Permalink to &quot;Grundlegende Assertions&quot;">​</a></h2><h3 id="wahrheitswert-assertions" tabindex="-1">Wahrheitswert-Assertions <a class="header-anchor" href="#wahrheitswert-assertions" aria-label="Permalink to &quot;Wahrheitswert-Assertions&quot;">​</a></h3><div class="language-hyp vp-adaptive-theme line-numbers-mode"><button title="Copy Code" class="copy"></button><span class="lang">hyp</span><pre class="shiki shiki-themes github-light github-dark vp-code" tabindex="0"><code><span class="line"><span>Focus {</span></span>
<span class="line"><span>    entrance {</span></span>
<span class="line"><span>        induce isLoggedIn = true;</span></span>
<span class="line"><span>        induce hasPermission = false;</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        // Einfache Wahrheitswert-Assertions</span></span>
<span class="line"><span>        assert isLoggedIn &quot;Benutzer muss eingeloggt sein&quot;;</span></span>
<span class="line"><span>        assert !hasPermission &quot;Benutzer sollte keine Berechtigung haben&quot;;</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        // Komplexe Bedingungen</span></span>
<span class="line"><span>        induce userAge = 25;</span></span>
<span class="line"><span>        induce isAdult = userAge &gt;= 18;</span></span>
<span class="line"><span>        assert isAdult &quot;Benutzer muss volljährig sein&quot;;</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        observe &quot;Alle Assertions bestanden!&quot;;</span></span>
<span class="line"><span>    }</span></span>
<span class="line"><span>} Relax;</span></span></code></pre><div class="line-numbers-wrapper" aria-hidden="true"><span class="line-number">1</span><br><span class="line-number">2</span><br><span class="line-number">3</span><br><span class="line-number">4</span><br><span class="line-number">5</span><br><span class="line-number">6</span><br><span class="line-number">7</span><br><span class="line-number">8</span><br><span class="line-number">9</span><br><span class="line-number">10</span><br><span class="line-number">11</span><br><span class="line-number">12</span><br><span class="line-number">13</span><br><span class="line-number">14</span><br><span class="line-number">15</span><br><span class="line-number">16</span><br><span class="line-number">17</span><br></div></div><h3 id="gleichheits-assertions" tabindex="-1">Gleichheits-Assertions <a class="header-anchor" href="#gleichheits-assertions" aria-label="Permalink to &quot;Gleichheits-Assertions&quot;">​</a></h3><div class="language-hyp vp-adaptive-theme line-numbers-mode"><button title="Copy Code" class="copy"></button><span class="lang">hyp</span><pre class="shiki shiki-themes github-light github-dark vp-code" tabindex="0"><code><span class="line"><span>Focus {</span></span>
<span class="line"><span>    entrance {</span></span>
<span class="line"><span>        induce expected = 42;</span></span>
<span class="line"><span>        induce actual = 42;</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        // Gleichheit prüfen</span></span>
<span class="line"><span>        assert actual == expected &quot;Wert sollte 42 sein&quot;;</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        // Ungleichheit prüfen</span></span>
<span class="line"><span>        induce differentValue = 100;</span></span>
<span class="line"><span>        assert actual != differentValue &quot;Werte sollten unterschiedlich sein&quot;;</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        // String-Gleichheit</span></span>
<span class="line"><span>        induce name = &quot;Alice&quot;;</span></span>
<span class="line"><span>        assert name == &quot;Alice&quot; &quot;Name sollte Alice sein&quot;;</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        observe &quot;Gleichheits-Assertions bestanden!&quot;;</span></span>
<span class="line"><span>    }</span></span>
<span class="line"><span>} Relax;</span></span></code></pre><div class="line-numbers-wrapper" aria-hidden="true"><span class="line-number">1</span><br><span class="line-number">2</span><br><span class="line-number">3</span><br><span class="line-number">4</span><br><span class="line-number">5</span><br><span class="line-number">6</span><br><span class="line-number">7</span><br><span class="line-number">8</span><br><span class="line-number">9</span><br><span class="line-number">10</span><br><span class="line-number">11</span><br><span class="line-number">12</span><br><span class="line-number">13</span><br><span class="line-number">14</span><br><span class="line-number">15</span><br><span class="line-number">16</span><br><span class="line-number">17</span><br><span class="line-number">18</span><br><span class="line-number">19</span><br></div></div><h3 id="numerische-assertions" tabindex="-1">Numerische Assertions <a class="header-anchor" href="#numerische-assertions" aria-label="Permalink to &quot;Numerische Assertions&quot;">​</a></h3><div class="language-hyp vp-adaptive-theme line-numbers-mode"><button title="Copy Code" class="copy"></button><span class="lang">hyp</span><pre class="shiki shiki-themes github-light github-dark vp-code" tabindex="0"><code><span class="line"><span>Focus {</span></span>
<span class="line"><span>    entrance {</span></span>
<span class="line"><span>        induce value = 50;</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        // Größer-als</span></span>
<span class="line"><span>        assert value &gt; 0 &quot;Wert sollte positiv sein&quot;;</span></span>
<span class="line"><span>        assert value &gt;= 50 &quot;Wert sollte mindestens 50 sein&quot;;</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        // Kleiner-als</span></span>
<span class="line"><span>        assert value &lt; 100 &quot;Wert sollte kleiner als 100 sein&quot;;</span></span>
<span class="line"><span>        assert value &lt;= 50 &quot;Wert sollte maximal 50 sein&quot;;</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        // Bereich prüfen</span></span>
<span class="line"><span>        assert value &gt;= 0 &amp;&amp; value &lt;= 100 &quot;Wert sollte zwischen 0 und 100 liegen&quot;;</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        observe &quot;Numerische Assertions bestanden!&quot;;</span></span>
<span class="line"><span>    }</span></span>
<span class="line"><span>} Relax;</span></span></code></pre><div class="line-numbers-wrapper" aria-hidden="true"><span class="line-number">1</span><br><span class="line-number">2</span><br><span class="line-number">3</span><br><span class="line-number">4</span><br><span class="line-number">5</span><br><span class="line-number">6</span><br><span class="line-number">7</span><br><span class="line-number">8</span><br><span class="line-number">9</span><br><span class="line-number">10</span><br><span class="line-number">11</span><br><span class="line-number">12</span><br><span class="line-number">13</span><br><span class="line-number">14</span><br><span class="line-number">15</span><br><span class="line-number">16</span><br><span class="line-number">17</span><br><span class="line-number">18</span><br></div></div><h2 id="erweiterte-assertions" tabindex="-1">Erweiterte Assertions <a class="header-anchor" href="#erweiterte-assertions" aria-label="Permalink to &quot;Erweiterte Assertions&quot;">​</a></h2><h3 id="array-assertions" tabindex="-1">Array-Assertions <a class="header-anchor" href="#array-assertions" aria-label="Permalink to &quot;Array-Assertions&quot;">​</a></h3><div class="language-hyp vp-adaptive-theme line-numbers-mode"><button title="Copy Code" class="copy"></button><span class="lang">hyp</span><pre class="shiki shiki-themes github-light github-dark vp-code" tabindex="0"><code><span class="line"><span>Focus {</span></span>
<span class="line"><span>    entrance {</span></span>
<span class="line"><span>        induce numbers = [1, 2, 3, 4, 5];</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        // Array-Länge prüfen</span></span>
<span class="line"><span>        assert ArrayLength(numbers) == 5 &quot;Array sollte 5 Elemente haben&quot;;</span></span>
<span class="line"><span>        assert ArrayLength(numbers) &gt; 0 &quot;Array sollte nicht leer sein&quot;;</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        // Array-Inhalt prüfen</span></span>
<span class="line"><span>        assert ArrayContains(numbers, 3) &quot;Array sollte 3 enthalten&quot;;</span></span>
<span class="line"><span>        assert !ArrayContains(numbers, 10) &quot;Array sollte 10 nicht enthalten&quot;;</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        // Array-Elemente prüfen</span></span>
<span class="line"><span>        assert ArrayGet(numbers, 0) == 1 &quot;Erstes Element sollte 1 sein&quot;;</span></span>
<span class="line"><span>        assert ArrayGet(numbers, ArrayLength(numbers) - 1) == 5 &quot;Letztes Element sollte 5 sein&quot;;</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        observe &quot;Array-Assertions bestanden!&quot;;</span></span>
<span class="line"><span>    }</span></span>
<span class="line"><span>} Relax;</span></span></code></pre><div class="line-numbers-wrapper" aria-hidden="true"><span class="line-number">1</span><br><span class="line-number">2</span><br><span class="line-number">3</span><br><span class="line-number">4</span><br><span class="line-number">5</span><br><span class="line-number">6</span><br><span class="line-number">7</span><br><span class="line-number">8</span><br><span class="line-number">9</span><br><span class="line-number">10</span><br><span class="line-number">11</span><br><span class="line-number">12</span><br><span class="line-number">13</span><br><span class="line-number">14</span><br><span class="line-number">15</span><br><span class="line-number">16</span><br><span class="line-number">17</span><br><span class="line-number">18</span><br><span class="line-number">19</span><br></div></div><h3 id="string-assertions" tabindex="-1">String-Assertions <a class="header-anchor" href="#string-assertions" aria-label="Permalink to &quot;String-Assertions&quot;">​</a></h3><div class="language-hyp vp-adaptive-theme line-numbers-mode"><button title="Copy Code" class="copy"></button><span class="lang">hyp</span><pre class="shiki shiki-themes github-light github-dark vp-code" tabindex="0"><code><span class="line"><span>Focus {</span></span>
<span class="line"><span>    entrance {</span></span>
<span class="line"><span>        induce text = &quot;Hello World&quot;;</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        // String-Länge</span></span>
<span class="line"><span>        assert Length(text) &gt; 0 &quot;Text sollte nicht leer sein&quot;;</span></span>
<span class="line"><span>        assert Length(text) &lt;= 100 &quot;Text sollte maximal 100 Zeichen haben&quot;;</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        // String-Inhalt</span></span>
<span class="line"><span>        assert Contains(text, &quot;Hello&quot;) &quot;Text sollte &#39;Hello&#39; enthalten&quot;;</span></span>
<span class="line"><span>        assert StartsWith(text, &quot;Hello&quot;) &quot;Text sollte mit &#39;Hello&#39; beginnen&quot;;</span></span>
<span class="line"><span>        assert EndsWith(text, &quot;World&quot;) &quot;Text sollte mit &#39;World&#39; enden&quot;;</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        // String-Format</span></span>
<span class="line"><span>        induce email = &quot;user@example.com&quot;;</span></span>
<span class="line"><span>        assert IsValidEmail(email) &quot;E-Mail sollte gültig sein&quot;;</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        observe &quot;String-Assertions bestanden!&quot;;</span></span>
<span class="line"><span>    }</span></span>
<span class="line"><span>} Relax;</span></span></code></pre><div class="line-numbers-wrapper" aria-hidden="true"><span class="line-number">1</span><br><span class="line-number">2</span><br><span class="line-number">3</span><br><span class="line-number">4</span><br><span class="line-number">5</span><br><span class="line-number">6</span><br><span class="line-number">7</span><br><span class="line-number">8</span><br><span class="line-number">9</span><br><span class="line-number">10</span><br><span class="line-number">11</span><br><span class="line-number">12</span><br><span class="line-number">13</span><br><span class="line-number">14</span><br><span class="line-number">15</span><br><span class="line-number">16</span><br><span class="line-number">17</span><br><span class="line-number">18</span><br><span class="line-number">19</span><br><span class="line-number">20</span><br></div></div><h3 id="objekt-assertions" tabindex="-1">Objekt-Assertions <a class="header-anchor" href="#objekt-assertions" aria-label="Permalink to &quot;Objekt-Assertions&quot;">​</a></h3><div class="language-hyp vp-adaptive-theme line-numbers-mode"><button title="Copy Code" class="copy"></button><span class="lang">hyp</span><pre class="shiki shiki-themes github-light github-dark vp-code" tabindex="0"><code><span class="line"><span>Focus {</span></span>
<span class="line"><span>    entrance {</span></span>
<span class="line"><span>        record Person {</span></span>
<span class="line"><span>            name: string;</span></span>
<span class="line"><span>            age: number;</span></span>
<span class="line"><span>        }</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        induce person = Person {</span></span>
<span class="line"><span>            name: &quot;Alice&quot;,</span></span>
<span class="line"><span>            age: 30</span></span>
<span class="line"><span>        };</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        // Objekt-Eigenschaften prüfen</span></span>
<span class="line"><span>        assert person.name != &quot;&quot; &quot;Name sollte nicht leer sein&quot;;</span></span>
<span class="line"><span>        assert person.age &gt;= 0 &quot;Alter sollte nicht negativ sein&quot;;</span></span>
<span class="line"><span>        assert person.age &lt;= 150 &quot;Alter sollte realistisch sein&quot;;</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        // Objekt-Typ prüfen</span></span>
<span class="line"><span>        assert person != null &quot;Person sollte nicht null sein&quot;;</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        observe &quot;Objekt-Assertions bestanden!&quot;;</span></span>
<span class="line"><span>    }</span></span>
<span class="line"><span>} Relax;</span></span></code></pre><div class="line-numbers-wrapper" aria-hidden="true"><span class="line-number">1</span><br><span class="line-number">2</span><br><span class="line-number">3</span><br><span class="line-number">4</span><br><span class="line-number">5</span><br><span class="line-number">6</span><br><span class="line-number">7</span><br><span class="line-number">8</span><br><span class="line-number">9</span><br><span class="line-number">10</span><br><span class="line-number">11</span><br><span class="line-number">12</span><br><span class="line-number">13</span><br><span class="line-number">14</span><br><span class="line-number">15</span><br><span class="line-number">16</span><br><span class="line-number">17</span><br><span class="line-number">18</span><br><span class="line-number">19</span><br><span class="line-number">20</span><br><span class="line-number">21</span><br><span class="line-number">22</span><br><span class="line-number">23</span><br></div></div><h2 id="spezialisierte-assertions" tabindex="-1">Spezialisierte Assertions <a class="header-anchor" href="#spezialisierte-assertions" aria-label="Permalink to &quot;Spezialisierte Assertions&quot;">​</a></h2><h3 id="typ-assertions" tabindex="-1">Typ-Assertions <a class="header-anchor" href="#typ-assertions" aria-label="Permalink to &quot;Typ-Assertions&quot;">​</a></h3><div class="language-hyp vp-adaptive-theme line-numbers-mode"><button title="Copy Code" class="copy"></button><span class="lang">hyp</span><pre class="shiki shiki-themes github-light github-dark vp-code" tabindex="0"><code><span class="line"><span>Focus {</span></span>
<span class="line"><span>    entrance {</span></span>
<span class="line"><span>        induce value = 42;</span></span>
<span class="line"><span>        induce text = &quot;Hello&quot;;</span></span>
<span class="line"><span>        induce array = [1, 2, 3];</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        // Typ prüfen</span></span>
<span class="line"><span>        assert TypeOf(value) == &quot;number&quot; &quot;Wert sollte vom Typ number sein&quot;;</span></span>
<span class="line"><span>        assert TypeOf(text) == &quot;string&quot; &quot;Text sollte vom Typ string sein&quot;;</span></span>
<span class="line"><span>        assert TypeOf(array) == &quot;array&quot; &quot;Array sollte vom Typ array sein&quot;;</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        // Null-Check</span></span>
<span class="line"><span>        induce nullableValue = null;</span></span>
<span class="line"><span>        assert nullableValue == null &quot;Wert sollte null sein&quot;;</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        observe &quot;Typ-Assertions bestanden!&quot;;</span></span>
<span class="line"><span>    }</span></span>
<span class="line"><span>} Relax;</span></span></code></pre><div class="line-numbers-wrapper" aria-hidden="true"><span class="line-number">1</span><br><span class="line-number">2</span><br><span class="line-number">3</span><br><span class="line-number">4</span><br><span class="line-number">5</span><br><span class="line-number">6</span><br><span class="line-number">7</span><br><span class="line-number">8</span><br><span class="line-number">9</span><br><span class="line-number">10</span><br><span class="line-number">11</span><br><span class="line-number">12</span><br><span class="line-number">13</span><br><span class="line-number">14</span><br><span class="line-number">15</span><br><span class="line-number">16</span><br><span class="line-number">17</span><br><span class="line-number">18</span><br></div></div><h3 id="funktions-assertions" tabindex="-1">Funktions-Assertions <a class="header-anchor" href="#funktions-assertions" aria-label="Permalink to &quot;Funktions-Assertions&quot;">​</a></h3><div class="language-hyp vp-adaptive-theme line-numbers-mode"><button title="Copy Code" class="copy"></button><span class="lang">hyp</span><pre class="shiki shiki-themes github-light github-dark vp-code" tabindex="0"><code><span class="line"><span>Focus {</span></span>
<span class="line"><span>    entrance {</span></span>
<span class="line"><span>        // Funktion definieren</span></span>
<span class="line"><span>        suggestion add(a: number, b: number): number {</span></span>
<span class="line"><span>            awaken a + b;</span></span>
<span class="line"><span>        }</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        // Funktionsergebnis prüfen</span></span>
<span class="line"><span>        induce result = call add(2, 3);</span></span>
<span class="line"><span>        assert result == 5 &quot;2 + 3 sollte 5 ergeben&quot;;</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        // Funktionsverhalten prüfen</span></span>
<span class="line"><span>        induce zeroResult = call add(0, 0);</span></span>
<span class="line"><span>        assert zeroResult == 0 &quot;0 + 0 sollte 0 ergeben&quot;;</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        // Negative Zahlen</span></span>
<span class="line"><span>        induce negativeResult = call add(-1, -2);</span></span>
<span class="line"><span>        assert negativeResult == -3 &quot;-1 + (-2) sollte -3 ergeben&quot;;</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        observe &quot;Funktions-Assertions bestanden!&quot;;</span></span>
<span class="line"><span>    }</span></span>
<span class="line"><span>} Relax;</span></span></code></pre><div class="line-numbers-wrapper" aria-hidden="true"><span class="line-number">1</span><br><span class="line-number">2</span><br><span class="line-number">3</span><br><span class="line-number">4</span><br><span class="line-number">5</span><br><span class="line-number">6</span><br><span class="line-number">7</span><br><span class="line-number">8</span><br><span class="line-number">9</span><br><span class="line-number">10</span><br><span class="line-number">11</span><br><span class="line-number">12</span><br><span class="line-number">13</span><br><span class="line-number">14</span><br><span class="line-number">15</span><br><span class="line-number">16</span><br><span class="line-number">17</span><br><span class="line-number">18</span><br><span class="line-number">19</span><br><span class="line-number">20</span><br><span class="line-number">21</span><br><span class="line-number">22</span><br></div></div><h3 id="performance-assertions" tabindex="-1">Performance-Assertions <a class="header-anchor" href="#performance-assertions" aria-label="Permalink to &quot;Performance-Assertions&quot;">​</a></h3><div class="language-hyp vp-adaptive-theme line-numbers-mode"><button title="Copy Code" class="copy"></button><span class="lang">hyp</span><pre class="shiki shiki-themes github-light github-dark vp-code" tabindex="0"><code><span class="line"><span>Focus {</span></span>
<span class="line"><span>    entrance {</span></span>
<span class="line"><span>        // Performance messen</span></span>
<span class="line"><span>        induce startTime = GetCurrentTime();</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        // Operation durchführen</span></span>
<span class="line"><span>        induce sum = 0;</span></span>
<span class="line"><span>        for (induce i = 0; i &lt; 1000; induce i = i + 1) {</span></span>
<span class="line"><span>            sum = sum + i;</span></span>
<span class="line"><span>        }</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        induce endTime = GetCurrentTime();</span></span>
<span class="line"><span>        induce executionTime = (endTime - startTime) * 1000; // in ms</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        // Performance-Assertions</span></span>
<span class="line"><span>        assert executionTime &lt; 100 &quot;Operation sollte schneller als 100ms sein&quot;;</span></span>
<span class="line"><span>        assert sum == 499500 &quot;Summe sollte korrekt berechnet werden&quot;;</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        observe &quot;Performance-Assertions bestanden!&quot;;</span></span>
<span class="line"><span>        observe &quot;Ausführungszeit: &quot; + executionTime + &quot; ms&quot;;</span></span>
<span class="line"><span>    }</span></span>
<span class="line"><span>} Relax;</span></span></code></pre><div class="line-numbers-wrapper" aria-hidden="true"><span class="line-number">1</span><br><span class="line-number">2</span><br><span class="line-number">3</span><br><span class="line-number">4</span><br><span class="line-number">5</span><br><span class="line-number">6</span><br><span class="line-number">7</span><br><span class="line-number">8</span><br><span class="line-number">9</span><br><span class="line-number">10</span><br><span class="line-number">11</span><br><span class="line-number">12</span><br><span class="line-number">13</span><br><span class="line-number">14</span><br><span class="line-number">15</span><br><span class="line-number">16</span><br><span class="line-number">17</span><br><span class="line-number">18</span><br><span class="line-number">19</span><br><span class="line-number">20</span><br><span class="line-number">21</span><br><span class="line-number">22</span><br></div></div><h2 id="assertion-patterns" tabindex="-1">Assertion-Patterns <a class="header-anchor" href="#assertion-patterns" aria-label="Permalink to &quot;Assertion-Patterns&quot;">​</a></h2><h3 id="eingabevalidierung" tabindex="-1">Eingabevalidierung <a class="header-anchor" href="#eingabevalidierung" aria-label="Permalink to &quot;Eingabevalidierung&quot;">​</a></h3><div class="language-hyp vp-adaptive-theme line-numbers-mode"><button title="Copy Code" class="copy"></button><span class="lang">hyp</span><pre class="shiki shiki-themes github-light github-dark vp-code" tabindex="0"><code><span class="line"><span>Focus {</span></span>
<span class="line"><span>    entrance {</span></span>
<span class="line"><span>        suggestion validateUserInput(username: string, age: number): boolean {</span></span>
<span class="line"><span>            // Username-Validierung</span></span>
<span class="line"><span>            assert Length(username) &gt;= 3 &quot;Username sollte mindestens 3 Zeichen haben&quot;;</span></span>
<span class="line"><span>            assert Length(username) &lt;= 20 &quot;Username sollte maximal 20 Zeichen haben&quot;;</span></span>
<span class="line"><span>            assert !Contains(username, &quot; &quot;) &quot;Username sollte keine Leerzeichen enthalten&quot;;</span></span>
<span class="line"><span></span></span>
<span class="line"><span>            // Alters-Validierung</span></span>
<span class="line"><span>            assert age &gt;= 13 &quot;Benutzer sollte mindestens 13 Jahre alt sein&quot;;</span></span>
<span class="line"><span>            assert age &lt;= 120 &quot;Alter sollte realistisch sein&quot;;</span></span>
<span class="line"><span></span></span>
<span class="line"><span>            // Zusätzliche Validierungen</span></span>
<span class="line"><span>            assert IsValidUsername(username) &quot;Username sollte gültig sein&quot;;</span></span>
<span class="line"><span></span></span>
<span class="line"><span>            return true;</span></span>
<span class="line"><span>        }</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        // Validierung testen</span></span>
<span class="line"><span>        try {</span></span>
<span class="line"><span>            induce isValid = call validateUserInput(&quot;alice123&quot;, 25);</span></span>
<span class="line"><span>            assert isValid &quot;Eingabe sollte gültig sein&quot;;</span></span>
<span class="line"><span>            observe &quot;Eingabevalidierung erfolgreich!&quot;;</span></span>
<span class="line"><span>        } catch (error) {</span></span>
<span class="line"><span>            observe &quot;Validierungsfehler: &quot; + error;</span></span>
<span class="line"><span>        }</span></span>
<span class="line"><span>    }</span></span>
<span class="line"><span>} Relax;</span></span></code></pre><div class="line-numbers-wrapper" aria-hidden="true"><span class="line-number">1</span><br><span class="line-number">2</span><br><span class="line-number">3</span><br><span class="line-number">4</span><br><span class="line-number">5</span><br><span class="line-number">6</span><br><span class="line-number">7</span><br><span class="line-number">8</span><br><span class="line-number">9</span><br><span class="line-number">10</span><br><span class="line-number">11</span><br><span class="line-number">12</span><br><span class="line-number">13</span><br><span class="line-number">14</span><br><span class="line-number">15</span><br><span class="line-number">16</span><br><span class="line-number">17</span><br><span class="line-number">18</span><br><span class="line-number">19</span><br><span class="line-number">20</span><br><span class="line-number">21</span><br><span class="line-number">22</span><br><span class="line-number">23</span><br><span class="line-number">24</span><br><span class="line-number">25</span><br><span class="line-number">26</span><br><span class="line-number">27</span><br><span class="line-number">28</span><br></div></div><h3 id="zustandsvalidierung" tabindex="-1">Zustandsvalidierung <a class="header-anchor" href="#zustandsvalidierung" aria-label="Permalink to &quot;Zustandsvalidierung&quot;">​</a></h3><div class="language-hyp vp-adaptive-theme line-numbers-mode"><button title="Copy Code" class="copy"></button><span class="lang">hyp</span><pre class="shiki shiki-themes github-light github-dark vp-code" tabindex="0"><code><span class="line"><span>Focus {</span></span>
<span class="line"><span>    entrance {</span></span>
<span class="line"><span>        record GameState {</span></span>
<span class="line"><span>            playerHealth: number;</span></span>
<span class="line"><span>            score: number;</span></span>
<span class="line"><span>            level: number;</span></span>
<span class="line"><span>        }</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        induce gameState = GameState {</span></span>
<span class="line"><span>            playerHealth: 100,</span></span>
<span class="line"><span>            score: 1500,</span></span>
<span class="line"><span>            level: 3</span></span>
<span class="line"><span>        };</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        // Zustands-Assertions</span></span>
<span class="line"><span>        assert gameState.playerHealth &gt;= 0 &quot;Spieler-Gesundheit sollte nicht negativ sein&quot;;</span></span>
<span class="line"><span>        assert gameState.playerHealth &lt;= 100 &quot;Spieler-Gesundheit sollte maximal 100 sein&quot;;</span></span>
<span class="line"><span>        assert gameState.score &gt;= 0 &quot;Punktzahl sollte nicht negativ sein&quot;;</span></span>
<span class="line"><span>        assert gameState.level &gt;= 1 &quot;Level sollte mindestens 1 sein&quot;;</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        // Konsistenz prüfen</span></span>
<span class="line"><span>        assert gameState.playerHealth &gt; 0 || gameState.level == 1 &quot;Spieler sollte leben oder im ersten Level sein&quot;;</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        observe &quot;Zustandsvalidierung erfolgreich!&quot;;</span></span>
<span class="line"><span>    }</span></span>
<span class="line"><span>} Relax;</span></span></code></pre><div class="line-numbers-wrapper" aria-hidden="true"><span class="line-number">1</span><br><span class="line-number">2</span><br><span class="line-number">3</span><br><span class="line-number">4</span><br><span class="line-number">5</span><br><span class="line-number">6</span><br><span class="line-number">7</span><br><span class="line-number">8</span><br><span class="line-number">9</span><br><span class="line-number">10</span><br><span class="line-number">11</span><br><span class="line-number">12</span><br><span class="line-number">13</span><br><span class="line-number">14</span><br><span class="line-number">15</span><br><span class="line-number">16</span><br><span class="line-number">17</span><br><span class="line-number">18</span><br><span class="line-number">19</span><br><span class="line-number">20</span><br><span class="line-number">21</span><br><span class="line-number">22</span><br><span class="line-number">23</span><br><span class="line-number">24</span><br><span class="line-number">25</span><br><span class="line-number">26</span><br></div></div><h3 id="api-response-validierung" tabindex="-1">API-Response-Validierung <a class="header-anchor" href="#api-response-validierung" aria-label="Permalink to &quot;API-Response-Validierung&quot;">​</a></h3><div class="language-hyp vp-adaptive-theme line-numbers-mode"><button title="Copy Code" class="copy"></button><span class="lang">hyp</span><pre class="shiki shiki-themes github-light github-dark vp-code" tabindex="0"><code><span class="line"><span>Focus {</span></span>
<span class="line"><span>    entrance {</span></span>
<span class="line"><span>        record ApiResponse {</span></span>
<span class="line"><span>            status: number;</span></span>
<span class="line"><span>            data: object;</span></span>
<span class="line"><span>            message: string;</span></span>
<span class="line"><span>        }</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        // Simulierte API-Antwort</span></span>
<span class="line"><span>        induce response = ApiResponse {</span></span>
<span class="line"><span>            status: 200,</span></span>
<span class="line"><span>            data: {</span></span>
<span class="line"><span>                userId: 123,</span></span>
<span class="line"><span>                name: &quot;Alice&quot;</span></span>
<span class="line"><span>            },</span></span>
<span class="line"><span>            message: &quot;Success&quot;</span></span>
<span class="line"><span>        };</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        // Response-Validierung</span></span>
<span class="line"><span>        assert response.status &gt;= 200 &amp;&amp; response.status &lt; 300 &quot;Status sollte erfolgreich sein&quot;;</span></span>
<span class="line"><span>        assert response.data != null &quot;Daten sollten vorhanden sein&quot;;</span></span>
<span class="line"><span>        assert Length(response.message) &gt; 0 &quot;Nachricht sollte nicht leer sein&quot;;</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        // Daten-Validierung</span></span>
<span class="line"><span>        if (response.data.userId) {</span></span>
<span class="line"><span>            assert response.data.userId &gt; 0 &quot;User-ID sollte positiv sein&quot;;</span></span>
<span class="line"><span>        }</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        if (response.data.name) {</span></span>
<span class="line"><span>            assert Length(response.data.name) &gt; 0 &quot;Name sollte nicht leer sein&quot;;</span></span>
<span class="line"><span>        }</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        observe &quot;API-Response-Validierung erfolgreich!&quot;;</span></span>
<span class="line"><span>    }</span></span>
<span class="line"><span>} Relax;</span></span></code></pre><div class="line-numbers-wrapper" aria-hidden="true"><span class="line-number">1</span><br><span class="line-number">2</span><br><span class="line-number">3</span><br><span class="line-number">4</span><br><span class="line-number">5</span><br><span class="line-number">6</span><br><span class="line-number">7</span><br><span class="line-number">8</span><br><span class="line-number">9</span><br><span class="line-number">10</span><br><span class="line-number">11</span><br><span class="line-number">12</span><br><span class="line-number">13</span><br><span class="line-number">14</span><br><span class="line-number">15</span><br><span class="line-number">16</span><br><span class="line-number">17</span><br><span class="line-number">18</span><br><span class="line-number">19</span><br><span class="line-number">20</span><br><span class="line-number">21</span><br><span class="line-number">22</span><br><span class="line-number">23</span><br><span class="line-number">24</span><br><span class="line-number">25</span><br><span class="line-number">26</span><br><span class="line-number">27</span><br><span class="line-number">28</span><br><span class="line-number">29</span><br><span class="line-number">30</span><br><span class="line-number">31</span><br><span class="line-number">32</span><br><span class="line-number">33</span><br><span class="line-number">34</span><br><span class="line-number">35</span><br></div></div><h2 id="assertion-frameworks" tabindex="-1">Assertion-Frameworks <a class="header-anchor" href="#assertion-frameworks" aria-label="Permalink to &quot;Assertion-Frameworks&quot;">​</a></h2><h3 id="test-assertions" tabindex="-1">Test-Assertions <a class="header-anchor" href="#test-assertions" aria-label="Permalink to &quot;Test-Assertions&quot;">​</a></h3><div class="language-hyp vp-adaptive-theme line-numbers-mode"><button title="Copy Code" class="copy"></button><span class="lang">hyp</span><pre class="shiki shiki-themes github-light github-dark vp-code" tabindex="0"><code><span class="line"><span>Focus {</span></span>
<span class="line"><span>    entrance {</span></span>
<span class="line"><span>        // Test-Setup</span></span>
<span class="line"><span>        induce testResults = [];</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        // Test-Funktionen</span></span>
<span class="line"><span>        suggestion assertEqual(actual: object, expected: object, message: string) {</span></span>
<span class="line"><span>            if (actual != expected) {</span></span>
<span class="line"><span>                ArrayPush(testResults, &quot;FAIL: &quot; + message + &quot; (Expected: &quot; + expected + &quot;, Got: &quot; + actual + &quot;)&quot;);</span></span>
<span class="line"><span>                throw &quot;Assertion failed: &quot; + message;</span></span>
<span class="line"><span>            } else {</span></span>
<span class="line"><span>                ArrayPush(testResults, &quot;PASS: &quot; + message);</span></span>
<span class="line"><span>            }</span></span>
<span class="line"><span>        }</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        suggestion assertTrue(condition: boolean, message: string) {</span></span>
<span class="line"><span>            if (!condition) {</span></span>
<span class="line"><span>                ArrayPush(testResults, &quot;FAIL: &quot; + message);</span></span>
<span class="line"><span>                throw &quot;Assertion failed: &quot; + message;</span></span>
<span class="line"><span>            } else {</span></span>
<span class="line"><span>                ArrayPush(testResults, &quot;PASS: &quot; + message);</span></span>
<span class="line"><span>            }</span></span>
<span class="line"><span>        }</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        // Tests ausführen</span></span>
<span class="line"><span>        try {</span></span>
<span class="line"><span>            call assertEqual(2 + 2, 4, &quot;Addition test&quot;);</span></span>
<span class="line"><span>            call assertTrue(Length(&quot;Hello&quot;) == 5, &quot;String length test&quot;);</span></span>
<span class="line"><span>            call assertEqual(ArrayLength([1, 2, 3]), 3, &quot;Array length test&quot;);</span></span>
<span class="line"><span></span></span>
<span class="line"><span>            observe &quot;Alle Tests bestanden!&quot;;</span></span>
<span class="line"><span>        } catch (error) {</span></span>
<span class="line"><span>            observe &quot;Test fehlgeschlagen: &quot; + error;</span></span>
<span class="line"><span>        }</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        // Test-Ergebnisse anzeigen</span></span>
<span class="line"><span>        observe &quot;Test-Ergebnisse:&quot;;</span></span>
<span class="line"><span>        for (induce i = 0; i &lt; ArrayLength(testResults); induce i = i + 1) {</span></span>
<span class="line"><span>            observe &quot;  &quot; + testResults[i];</span></span>
<span class="line"><span>        }</span></span>
<span class="line"><span>    }</span></span>
<span class="line"><span>} Relax;</span></span></code></pre><div class="line-numbers-wrapper" aria-hidden="true"><span class="line-number">1</span><br><span class="line-number">2</span><br><span class="line-number">3</span><br><span class="line-number">4</span><br><span class="line-number">5</span><br><span class="line-number">6</span><br><span class="line-number">7</span><br><span class="line-number">8</span><br><span class="line-number">9</span><br><span class="line-number">10</span><br><span class="line-number">11</span><br><span class="line-number">12</span><br><span class="line-number">13</span><br><span class="line-number">14</span><br><span class="line-number">15</span><br><span class="line-number">16</span><br><span class="line-number">17</span><br><span class="line-number">18</span><br><span class="line-number">19</span><br><span class="line-number">20</span><br><span class="line-number">21</span><br><span class="line-number">22</span><br><span class="line-number">23</span><br><span class="line-number">24</span><br><span class="line-number">25</span><br><span class="line-number">26</span><br><span class="line-number">27</span><br><span class="line-number">28</span><br><span class="line-number">29</span><br><span class="line-number">30</span><br><span class="line-number">31</span><br><span class="line-number">32</span><br><span class="line-number">33</span><br><span class="line-number">34</span><br><span class="line-number">35</span><br><span class="line-number">36</span><br><span class="line-number">37</span><br><span class="line-number">38</span><br><span class="line-number">39</span><br><span class="line-number">40</span><br><span class="line-number">41</span><br><span class="line-number">42</span><br></div></div><h3 id="debug-assertions" tabindex="-1">Debug-Assertions <a class="header-anchor" href="#debug-assertions" aria-label="Permalink to &quot;Debug-Assertions&quot;">​</a></h3><div class="language-hyp vp-adaptive-theme line-numbers-mode"><button title="Copy Code" class="copy"></button><span class="lang">hyp</span><pre class="shiki shiki-themes github-light github-dark vp-code" tabindex="0"><code><span class="line"><span>Focus {</span></span>
<span class="line"><span>    entrance {</span></span>
<span class="line"><span>        induce debugMode = true;</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        suggestion debugAssert(condition: boolean, message: string) {</span></span>
<span class="line"><span>            if (debugMode &amp;&amp; !condition) {</span></span>
<span class="line"><span>                observe &quot;[DEBUG] Assertion failed: &quot; + message;</span></span>
<span class="line"><span>                observe &quot;[DEBUG] Stack trace: &quot; + GetCallStack();</span></span>
<span class="line"><span>            }</span></span>
<span class="line"><span>        }</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        // Debug-Assertions verwenden</span></span>
<span class="line"><span>        induce value = 42;</span></span>
<span class="line"><span>        call debugAssert(value &gt; 0, &quot;Wert sollte positiv sein&quot;);</span></span>
<span class="line"><span>        call debugAssert(value &lt; 100, &quot;Wert sollte kleiner als 100 sein&quot;);</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        // Debug-Informationen sammeln</span></span>
<span class="line"><span>        if (debugMode) {</span></span>
<span class="line"><span>            induce memoryUsage = GetMemoryUsage();</span></span>
<span class="line"><span>            call debugAssert(memoryUsage &lt; 1000, &quot;Speichernutzung sollte unter 1GB sein&quot;);</span></span>
<span class="line"><span>        }</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        observe &quot;Debug-Assertions abgeschlossen!&quot;;</span></span>
<span class="line"><span>    }</span></span>
<span class="line"><span>} Relax;</span></span></code></pre><div class="line-numbers-wrapper" aria-hidden="true"><span class="line-number">1</span><br><span class="line-number">2</span><br><span class="line-number">3</span><br><span class="line-number">4</span><br><span class="line-number">5</span><br><span class="line-number">6</span><br><span class="line-number">7</span><br><span class="line-number">8</span><br><span class="line-number">9</span><br><span class="line-number">10</span><br><span class="line-number">11</span><br><span class="line-number">12</span><br><span class="line-number">13</span><br><span class="line-number">14</span><br><span class="line-number">15</span><br><span class="line-number">16</span><br><span class="line-number">17</span><br><span class="line-number">18</span><br><span class="line-number">19</span><br><span class="line-number">20</span><br><span class="line-number">21</span><br><span class="line-number">22</span><br><span class="line-number">23</span><br><span class="line-number">24</span><br><span class="line-number">25</span><br></div></div><h2 id="best-practices" tabindex="-1">Best Practices <a class="header-anchor" href="#best-practices" aria-label="Permalink to &quot;Best Practices&quot;">​</a></h2><h3 id="assertion-strategien" tabindex="-1">Assertion-Strategien <a class="header-anchor" href="#assertion-strategien" aria-label="Permalink to &quot;Assertion-Strategien&quot;">​</a></h3><div class="language-hyp vp-adaptive-theme line-numbers-mode"><button title="Copy Code" class="copy"></button><span class="lang">hyp</span><pre class="shiki shiki-themes github-light github-dark vp-code" tabindex="0"><code><span class="line"><span>Focus {</span></span>
<span class="line"><span>    entrance {</span></span>
<span class="line"><span>        // ✅ GUT: Spezifische Assertions</span></span>
<span class="line"><span>        induce userAge = 25;</span></span>
<span class="line"><span>        assert userAge &gt;= 18 &quot;Benutzer muss volljährig sein&quot;;</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        // ✅ GUT: Aussagekräftige Nachrichten</span></span>
<span class="line"><span>        induce result = 42;</span></span>
<span class="line"><span>        assert result == 42 &quot;Berechnung sollte 42 ergeben, nicht &quot; + result;</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        // ✅ GUT: Frühe Validierung</span></span>
<span class="line"><span>        suggestion processUser(user: object) {</span></span>
<span class="line"><span>            assert user != null &quot;Benutzer-Objekt darf nicht null sein&quot;;</span></span>
<span class="line"><span>            assert user.name != &quot;&quot; &quot;Benutzername darf nicht leer sein&quot;;</span></span>
<span class="line"><span></span></span>
<span class="line"><span>            // Verarbeitung...</span></span>
<span class="line"><span>        }</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        // ❌ SCHLECHT: Zu allgemeine Assertions</span></span>
<span class="line"><span>        assert true &quot;Alles ist gut&quot;;</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        // ❌ SCHLECHT: Fehlende Nachrichten</span></span>
<span class="line"><span>        assert userAge &gt;= 18;</span></span>
<span class="line"><span>    }</span></span>
<span class="line"><span>} Relax;</span></span></code></pre><div class="line-numbers-wrapper" aria-hidden="true"><span class="line-number">1</span><br><span class="line-number">2</span><br><span class="line-number">3</span><br><span class="line-number">4</span><br><span class="line-number">5</span><br><span class="line-number">6</span><br><span class="line-number">7</span><br><span class="line-number">8</span><br><span class="line-number">9</span><br><span class="line-number">10</span><br><span class="line-number">11</span><br><span class="line-number">12</span><br><span class="line-number">13</span><br><span class="line-number">14</span><br><span class="line-number">15</span><br><span class="line-number">16</span><br><span class="line-number">17</span><br><span class="line-number">18</span><br><span class="line-number">19</span><br><span class="line-number">20</span><br><span class="line-number">21</span><br><span class="line-number">22</span><br><span class="line-number">23</span><br><span class="line-number">24</span><br><span class="line-number">25</span><br></div></div><h3 id="performance-considerations" tabindex="-1">Performance-Considerations <a class="header-anchor" href="#performance-considerations" aria-label="Permalink to &quot;Performance-Considerations&quot;">​</a></h3><div class="language-hyp vp-adaptive-theme line-numbers-mode"><button title="Copy Code" class="copy"></button><span class="lang">hyp</span><pre class="shiki shiki-themes github-light github-dark vp-code" tabindex="0"><code><span class="line"><span>Focus {</span></span>
<span class="line"><span>    entrance {</span></span>
<span class="line"><span>        // ✅ GUT: Einfache Assertions für Performance-kritische Pfade</span></span>
<span class="line"><span>        induce criticalValue = 100;</span></span>
<span class="line"><span>        assert criticalValue &gt; 0; // Schnelle Prüfung</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        // ✅ GUT: Komplexe Assertions nur im Debug-Modus</span></span>
<span class="line"><span>        induce debugMode = true;</span></span>
<span class="line"><span>        if (debugMode) {</span></span>
<span class="line"><span>            induce complexValidation = ValidateComplexData();</span></span>
<span class="line"><span>            assert complexValidation &quot;Komplexe Validierung fehlgeschlagen&quot;;</span></span>
<span class="line"><span>        }</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        // ✅ GUT: Assertions für invariante Bedingungen</span></span>
<span class="line"><span>        induce loopCount = 0;</span></span>
<span class="line"><span>        while (loopCount &lt; 10) {</span></span>
<span class="line"><span>            assert loopCount &gt;= 0 &quot;Schleifenzähler sollte nicht negativ sein&quot;;</span></span>
<span class="line"><span>            loopCount = loopCount + 1;</span></span>
<span class="line"><span>        }</span></span>
<span class="line"><span>    }</span></span>
<span class="line"><span>} Relax;</span></span></code></pre><div class="line-numbers-wrapper" aria-hidden="true"><span class="line-number">1</span><br><span class="line-number">2</span><br><span class="line-number">3</span><br><span class="line-number">4</span><br><span class="line-number">5</span><br><span class="line-number">6</span><br><span class="line-number">7</span><br><span class="line-number">8</span><br><span class="line-number">9</span><br><span class="line-number">10</span><br><span class="line-number">11</span><br><span class="line-number">12</span><br><span class="line-number">13</span><br><span class="line-number">14</span><br><span class="line-number">15</span><br><span class="line-number">16</span><br><span class="line-number">17</span><br><span class="line-number">18</span><br><span class="line-number">19</span><br><span class="line-number">20</span><br><span class="line-number">21</span><br></div></div><h2 id="fehlerbehandlung" tabindex="-1">Fehlerbehandlung <a class="header-anchor" href="#fehlerbehandlung" aria-label="Permalink to &quot;Fehlerbehandlung&quot;">​</a></h2><h3 id="assertion-fehler-abfangen" tabindex="-1">Assertion-Fehler abfangen <a class="header-anchor" href="#assertion-fehler-abfangen" aria-label="Permalink to &quot;Assertion-Fehler abfangen&quot;">​</a></h3><div class="language-hyp vp-adaptive-theme line-numbers-mode"><button title="Copy Code" class="copy"></button><span class="lang">hyp</span><pre class="shiki shiki-themes github-light github-dark vp-code" tabindex="0"><code><span class="line"><span>Focus {</span></span>
<span class="line"><span>    entrance {</span></span>
<span class="line"><span>        induce assertionErrors = [];</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        suggestion safeAssert(condition: boolean, message: string) {</span></span>
<span class="line"><span>            try {</span></span>
<span class="line"><span>                assert condition message;</span></span>
<span class="line"><span>                return true;</span></span>
<span class="line"><span>            } catch (error) {</span></span>
<span class="line"><span>                ArrayPush(assertionErrors, error);</span></span>
<span class="line"><span>                return false;</span></span>
<span class="line"><span>            }</span></span>
<span class="line"><span>        }</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        // Sichere Assertions verwenden</span></span>
<span class="line"><span>        induce test1 = call safeAssert(2 + 2 == 4, &quot;Mathematik funktioniert&quot;);</span></span>
<span class="line"><span>        induce test2 = call safeAssert(2 + 2 == 5, &quot;Diese Assertion sollte fehlschlagen&quot;);</span></span>
<span class="line"><span>        induce test3 = call safeAssert(Length(&quot;Hello&quot;) == 5, &quot;String-Länge ist korrekt&quot;);</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        // Ergebnisse auswerten</span></span>
<span class="line"><span>        observe &quot;Erfolgreiche Assertions: &quot; + (test1 &amp;&amp; test3);</span></span>
<span class="line"><span>        observe &quot;Fehlgeschlagene Assertions: &quot; + (!test2);</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        if (ArrayLength(assertionErrors) &gt; 0) {</span></span>
<span class="line"><span>            observe &quot;Assertion-Fehler:&quot;;</span></span>
<span class="line"><span>            for (induce i = 0; i &lt; ArrayLength(assertionErrors); induce i = i + 1) {</span></span>
<span class="line"><span>                observe &quot;  &quot; + assertionErrors[i];</span></span>
<span class="line"><span>            }</span></span>
<span class="line"><span>        }</span></span>
<span class="line"><span>    }</span></span>
<span class="line"><span>} Relax;</span></span></code></pre><div class="line-numbers-wrapper" aria-hidden="true"><span class="line-number">1</span><br><span class="line-number">2</span><br><span class="line-number">3</span><br><span class="line-number">4</span><br><span class="line-number">5</span><br><span class="line-number">6</span><br><span class="line-number">7</span><br><span class="line-number">8</span><br><span class="line-number">9</span><br><span class="line-number">10</span><br><span class="line-number">11</span><br><span class="line-number">12</span><br><span class="line-number">13</span><br><span class="line-number">14</span><br><span class="line-number">15</span><br><span class="line-number">16</span><br><span class="line-number">17</span><br><span class="line-number">18</span><br><span class="line-number">19</span><br><span class="line-number">20</span><br><span class="line-number">21</span><br><span class="line-number">22</span><br><span class="line-number">23</span><br><span class="line-number">24</span><br><span class="line-number">25</span><br><span class="line-number">26</span><br><span class="line-number">27</span><br><span class="line-number">28</span><br><span class="line-number">29</span><br><span class="line-number">30</span><br><span class="line-number">31</span><br></div></div><h3 id="assertion-level" tabindex="-1">Assertion-Level <a class="header-anchor" href="#assertion-level" aria-label="Permalink to &quot;Assertion-Level&quot;">​</a></h3><div class="language-hyp vp-adaptive-theme line-numbers-mode"><button title="Copy Code" class="copy"></button><span class="lang">hyp</span><pre class="shiki shiki-themes github-light github-dark vp-code" tabindex="0"><code><span class="line"><span>Focus {</span></span>
<span class="line"><span>    entrance {</span></span>
<span class="line"><span>        induce assertionLevel = &quot;strict&quot;; // &quot;strict&quot;, &quot;normal&quot;, &quot;relaxed&quot;</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        suggestion levelAssert(condition: boolean, message: string, level: string) {</span></span>
<span class="line"><span>            if (level == &quot;strict&quot; ||</span></span>
<span class="line"><span>                (level == &quot;normal&quot; &amp;&amp; assertionLevel != &quot;relaxed&quot;) ||</span></span>
<span class="line"><span>                (level == &quot;relaxed&quot; &amp;&amp; assertionLevel == &quot;relaxed&quot;)) {</span></span>
<span class="line"><span>                assert condition message;</span></span>
<span class="line"><span>            }</span></span>
<span class="line"><span>        }</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        // Level-spezifische Assertions</span></span>
<span class="line"><span>        call levelAssert(true, &quot;Immer prüfen&quot;, &quot;strict&quot;);</span></span>
<span class="line"><span>        call levelAssert(2 + 2 == 4, &quot;Normale Prüfung&quot;, &quot;normal&quot;);</span></span>
<span class="line"><span>        call levelAssert(Length(&quot;test&quot;) == 4, &quot;Entspannte Prüfung&quot;, &quot;relaxed&quot;);</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        observe &quot;Level-spezifische Assertions abgeschlossen!&quot;;</span></span>
<span class="line"><span>    }</span></span>
<span class="line"><span>} Relax;</span></span></code></pre><div class="line-numbers-wrapper" aria-hidden="true"><span class="line-number">1</span><br><span class="line-number">2</span><br><span class="line-number">3</span><br><span class="line-number">4</span><br><span class="line-number">5</span><br><span class="line-number">6</span><br><span class="line-number">7</span><br><span class="line-number">8</span><br><span class="line-number">9</span><br><span class="line-number">10</span><br><span class="line-number">11</span><br><span class="line-number">12</span><br><span class="line-number">13</span><br><span class="line-number">14</span><br><span class="line-number">15</span><br><span class="line-number">16</span><br><span class="line-number">17</span><br><span class="line-number">18</span><br><span class="line-number">19</span><br><span class="line-number">20</span><br></div></div><h2 id="nachste-schritte" tabindex="-1">Nächste Schritte <a class="header-anchor" href="#nachste-schritte" aria-label="Permalink to &quot;Nächste Schritte&quot;">​</a></h2><ul><li><a href="./../testing/overview.html">Testing Overview</a> - Umfassender Testing-Guide</li><li><a href="./functions.html">Functions</a> - Funktionsdefinitionen und -aufrufe</li><li><a href="./../error-handling/overview.html">Error Handling</a> - Fehlerbehandlung</li></ul><hr><p><strong>Assertions gemeistert? Dann lerne <a href="./../testing/overview.html">Testing Overview</a> kennen!</strong> ✅</p>`,56)])])}const d=n(l,[["render",r]]);export{m as __pageData,d as default};
