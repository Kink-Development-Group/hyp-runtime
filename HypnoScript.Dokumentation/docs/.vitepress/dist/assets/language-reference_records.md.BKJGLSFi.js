import{_ as s,c as a,o as e,ag as p}from"./chunks/framework.Dli2S8Ej.js";const m=JSON.parse('{"title":"Records","description":"","frontmatter":{"title":"Records"},"headers":[],"relativePath":"language-reference/records.md","filePath":"language-reference/records.md","lastUpdated":1750802436000}'),l={name:"language-reference/records.md"};function r(i,n,c,b,u,t){return e(),a("div",null,[...n[0]||(n[0]=[p(`<h1 id="records" tabindex="-1">Records <a class="header-anchor" href="#records" aria-label="Permalink to &quot;Records&quot;">​</a></h1><p>Records sind strukturierte Datentypen in HypnoScript, die es ermöglichen, zusammengehörige Daten in einem Objekt zu gruppieren.</p><h2 id="ubersicht" tabindex="-1">Übersicht <a class="header-anchor" href="#ubersicht" aria-label="Permalink to &quot;Übersicht&quot;">​</a></h2><p>Records sind unveränderliche (immutable) Datenstrukturen, die mehrere Felder mit verschiedenen Typen enthalten können. Sie sind ideal für die Darstellung von Entitäten, Konfigurationen und strukturierten Daten.</p><h2 id="syntax" tabindex="-1">Syntax <a class="header-anchor" href="#syntax" aria-label="Permalink to &quot;Syntax&quot;">​</a></h2><h3 id="record-deklaration" tabindex="-1">Record-Deklaration <a class="header-anchor" href="#record-deklaration" aria-label="Permalink to &quot;Record-Deklaration&quot;">​</a></h3><div class="language-hyp vp-adaptive-theme line-numbers-mode"><button title="Copy Code" class="copy"></button><span class="lang">hyp</span><pre class="shiki shiki-themes github-light github-dark vp-code" tabindex="0"><code><span class="line"><span>record Person {</span></span>
<span class="line"><span>    name: string;</span></span>
<span class="line"><span>    age: number;</span></span>
<span class="line"><span>    email: string;</span></span>
<span class="line"><span>    isActive: boolean;</span></span>
<span class="line"><span>}</span></span></code></pre><div class="line-numbers-wrapper" aria-hidden="true"><span class="line-number">1</span><br><span class="line-number">2</span><br><span class="line-number">3</span><br><span class="line-number">4</span><br><span class="line-number">5</span><br><span class="line-number">6</span><br></div></div><h3 id="record-instanziierung" tabindex="-1">Record-Instanziierung <a class="header-anchor" href="#record-instanziierung" aria-label="Permalink to &quot;Record-Instanziierung&quot;">​</a></h3><div class="language-hyp vp-adaptive-theme line-numbers-mode"><button title="Copy Code" class="copy"></button><span class="lang">hyp</span><pre class="shiki shiki-themes github-light github-dark vp-code" tabindex="0"><code><span class="line"><span>induce person = Person {</span></span>
<span class="line"><span>    name: &quot;Alice Johnson&quot;,</span></span>
<span class="line"><span>    age: 30,</span></span>
<span class="line"><span>    email: &quot;alice@example.com&quot;,</span></span>
<span class="line"><span>    isActive: true</span></span>
<span class="line"><span>};</span></span></code></pre><div class="line-numbers-wrapper" aria-hidden="true"><span class="line-number">1</span><br><span class="line-number">2</span><br><span class="line-number">3</span><br><span class="line-number">4</span><br><span class="line-number">5</span><br><span class="line-number">6</span><br></div></div><h3 id="record-mit-optionalen-feldern" tabindex="-1">Record mit optionalen Feldern <a class="header-anchor" href="#record-mit-optionalen-feldern" aria-label="Permalink to &quot;Record mit optionalen Feldern&quot;">​</a></h3><div class="language-hyp vp-adaptive-theme line-numbers-mode"><button title="Copy Code" class="copy"></button><span class="lang">hyp</span><pre class="shiki shiki-themes github-light github-dark vp-code" tabindex="0"><code><span class="line"><span>record User {</span></span>
<span class="line"><span>    id: number;</span></span>
<span class="line"><span>    username: string;</span></span>
<span class="line"><span>    email?: string;  // Optionales Feld</span></span>
<span class="line"><span>    lastLogin?: number;</span></span>
<span class="line"><span>}</span></span></code></pre><div class="line-numbers-wrapper" aria-hidden="true"><span class="line-number">1</span><br><span class="line-number">2</span><br><span class="line-number">3</span><br><span class="line-number">4</span><br><span class="line-number">5</span><br><span class="line-number">6</span><br></div></div><h2 id="grundlegende-verwendung" tabindex="-1">Grundlegende Verwendung <a class="header-anchor" href="#grundlegende-verwendung" aria-label="Permalink to &quot;Grundlegende Verwendung&quot;">​</a></h2><h3 id="einfacher-record" tabindex="-1">Einfacher Record <a class="header-anchor" href="#einfacher-record" aria-label="Permalink to &quot;Einfacher Record&quot;">​</a></h3><div class="language-hyp vp-adaptive-theme line-numbers-mode"><button title="Copy Code" class="copy"></button><span class="lang">hyp</span><pre class="shiki shiki-themes github-light github-dark vp-code" tabindex="0"><code><span class="line"><span>Focus {</span></span>
<span class="line"><span>    entrance {</span></span>
<span class="line"><span>        // Record definieren</span></span>
<span class="line"><span>        record Point {</span></span>
<span class="line"><span>            x: number;</span></span>
<span class="line"><span>            y: number;</span></span>
<span class="line"><span>        }</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        // Record-Instanz erstellen</span></span>
<span class="line"><span>        induce point1 = Point {</span></span>
<span class="line"><span>            x: 10,</span></span>
<span class="line"><span>            y: 20</span></span>
<span class="line"><span>        };</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        // Auf Felder zugreifen</span></span>
<span class="line"><span>        observe &quot;X-Koordinate: &quot; + point1.x;</span></span>
<span class="line"><span>        observe &quot;Y-Koordinate: &quot; + point1.y;</span></span>
<span class="line"><span>    }</span></span>
<span class="line"><span>} Relax;</span></span></code></pre><div class="line-numbers-wrapper" aria-hidden="true"><span class="line-number">1</span><br><span class="line-number">2</span><br><span class="line-number">3</span><br><span class="line-number">4</span><br><span class="line-number">5</span><br><span class="line-number">6</span><br><span class="line-number">7</span><br><span class="line-number">8</span><br><span class="line-number">9</span><br><span class="line-number">10</span><br><span class="line-number">11</span><br><span class="line-number">12</span><br><span class="line-number">13</span><br><span class="line-number">14</span><br><span class="line-number">15</span><br><span class="line-number">16</span><br><span class="line-number">17</span><br><span class="line-number">18</span><br><span class="line-number">19</span><br></div></div><h3 id="record-mit-verschiedenen-datentypen" tabindex="-1">Record mit verschiedenen Datentypen <a class="header-anchor" href="#record-mit-verschiedenen-datentypen" aria-label="Permalink to &quot;Record mit verschiedenen Datentypen&quot;">​</a></h3><div class="language-hyp vp-adaptive-theme line-numbers-mode"><button title="Copy Code" class="copy"></button><span class="lang">hyp</span><pre class="shiki shiki-themes github-light github-dark vp-code" tabindex="0"><code><span class="line"><span>Focus {</span></span>
<span class="line"><span>    entrance {</span></span>
<span class="line"><span>        record Product {</span></span>
<span class="line"><span>            id: number;</span></span>
<span class="line"><span>            name: string;</span></span>
<span class="line"><span>            price: number;</span></span>
<span class="line"><span>            categories: array;</span></span>
<span class="line"><span>            inStock: boolean;</span></span>
<span class="line"><span>            metadata: object;</span></span>
<span class="line"><span>        }</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        induce product = Product {</span></span>
<span class="line"><span>            id: 12345,</span></span>
<span class="line"><span>            name: &quot;HypnoScript Pro&quot;,</span></span>
<span class="line"><span>            price: 99.99,</span></span>
<span class="line"><span>            categories: [&quot;Software&quot;, &quot;Programming&quot;, &quot;Hypnosis&quot;],</span></span>
<span class="line"><span>            inStock: true,</span></span>
<span class="line"><span>            metadata: {</span></span>
<span class="line"><span>                version: &quot;1.0.0&quot;,</span></span>
<span class="line"><span>                releaseDate: &quot;2024-01-15&quot;</span></span>
<span class="line"><span>            }</span></span>
<span class="line"><span>        };</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        observe &quot;Produkt: &quot; + product.name;</span></span>
<span class="line"><span>        observe &quot;Preis: &quot; + product.price + &quot; €&quot;;</span></span>
<span class="line"><span>        observe &quot;Kategorien: &quot; + product.categories;</span></span>
<span class="line"><span>    }</span></span>
<span class="line"><span>} Relax;</span></span></code></pre><div class="line-numbers-wrapper" aria-hidden="true"><span class="line-number">1</span><br><span class="line-number">2</span><br><span class="line-number">3</span><br><span class="line-number">4</span><br><span class="line-number">5</span><br><span class="line-number">6</span><br><span class="line-number">7</span><br><span class="line-number">8</span><br><span class="line-number">9</span><br><span class="line-number">10</span><br><span class="line-number">11</span><br><span class="line-number">12</span><br><span class="line-number">13</span><br><span class="line-number">14</span><br><span class="line-number">15</span><br><span class="line-number">16</span><br><span class="line-number">17</span><br><span class="line-number">18</span><br><span class="line-number">19</span><br><span class="line-number">20</span><br><span class="line-number">21</span><br><span class="line-number">22</span><br><span class="line-number">23</span><br><span class="line-number">24</span><br><span class="line-number">25</span><br><span class="line-number">26</span><br><span class="line-number">27</span><br><span class="line-number">28</span><br></div></div><h2 id="record-operationen" tabindex="-1">Record-Operationen <a class="header-anchor" href="#record-operationen" aria-label="Permalink to &quot;Record-Operationen&quot;">​</a></h2><h3 id="feldzugriff" tabindex="-1">Feldzugriff <a class="header-anchor" href="#feldzugriff" aria-label="Permalink to &quot;Feldzugriff&quot;">​</a></h3><div class="language-hyp vp-adaptive-theme line-numbers-mode"><button title="Copy Code" class="copy"></button><span class="lang">hyp</span><pre class="shiki shiki-themes github-light github-dark vp-code" tabindex="0"><code><span class="line"><span>Focus {</span></span>
<span class="line"><span>    entrance {</span></span>
<span class="line"><span>        record Address {</span></span>
<span class="line"><span>            street: string;</span></span>
<span class="line"><span>            city: string;</span></span>
<span class="line"><span>            zipCode: string;</span></span>
<span class="line"><span>            country: string;</span></span>
<span class="line"><span>        }</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        induce address = Address {</span></span>
<span class="line"><span>            street: &quot;Musterstraße 123&quot;,</span></span>
<span class="line"><span>            city: &quot;Berlin&quot;,</span></span>
<span class="line"><span>            zipCode: &quot;10115&quot;,</span></span>
<span class="line"><span>            country: &quot;Deutschland&quot;</span></span>
<span class="line"><span>        };</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        // Direkter Feldzugriff</span></span>
<span class="line"><span>        observe &quot;Straße: &quot; + address.street;</span></span>
<span class="line"><span>        observe &quot;Stadt: &quot; + address.city;</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        // Dynamischer Feldzugriff</span></span>
<span class="line"><span>        induce fieldName = &quot;zipCode&quot;;</span></span>
<span class="line"><span>        induce fieldValue = address[fieldName];</span></span>
<span class="line"><span>        observe &quot;PLZ: &quot; + fieldValue;</span></span>
<span class="line"><span>    }</span></span>
<span class="line"><span>} Relax;</span></span></code></pre><div class="line-numbers-wrapper" aria-hidden="true"><span class="line-number">1</span><br><span class="line-number">2</span><br><span class="line-number">3</span><br><span class="line-number">4</span><br><span class="line-number">5</span><br><span class="line-number">6</span><br><span class="line-number">7</span><br><span class="line-number">8</span><br><span class="line-number">9</span><br><span class="line-number">10</span><br><span class="line-number">11</span><br><span class="line-number">12</span><br><span class="line-number">13</span><br><span class="line-number">14</span><br><span class="line-number">15</span><br><span class="line-number">16</span><br><span class="line-number">17</span><br><span class="line-number">18</span><br><span class="line-number">19</span><br><span class="line-number">20</span><br><span class="line-number">21</span><br><span class="line-number">22</span><br><span class="line-number">23</span><br><span class="line-number">24</span><br><span class="line-number">25</span><br><span class="line-number">26</span><br></div></div><h3 id="record-kopien-mit-anderungen" tabindex="-1">Record-Kopien mit Änderungen <a class="header-anchor" href="#record-kopien-mit-anderungen" aria-label="Permalink to &quot;Record-Kopien mit Änderungen&quot;">​</a></h3><div class="language-hyp vp-adaptive-theme line-numbers-mode"><button title="Copy Code" class="copy"></button><span class="lang">hyp</span><pre class="shiki shiki-themes github-light github-dark vp-code" tabindex="0"><code><span class="line"><span>Focus {</span></span>
<span class="line"><span>    entrance {</span></span>
<span class="line"><span>        record Config {</span></span>
<span class="line"><span>            theme: string;</span></span>
<span class="line"><span>            language: string;</span></span>
<span class="line"><span>            notifications: boolean;</span></span>
<span class="line"><span>        }</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        induce defaultConfig = Config {</span></span>
<span class="line"><span>            theme: &quot;dark&quot;,</span></span>
<span class="line"><span>            language: &quot;de&quot;,</span></span>
<span class="line"><span>            notifications: true</span></span>
<span class="line"><span>        };</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        // Kopie mit Änderungen erstellen</span></span>
<span class="line"><span>        induce userConfig = defaultConfig with {</span></span>
<span class="line"><span>            theme: &quot;light&quot;,</span></span>
<span class="line"><span>            language: &quot;en&quot;</span></span>
<span class="line"><span>        };</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        observe &quot;Standard-Theme: &quot; + defaultConfig.theme;</span></span>
<span class="line"><span>        observe &quot;Benutzer-Theme: &quot; + userConfig.theme;</span></span>
<span class="line"><span>    }</span></span>
<span class="line"><span>} Relax;</span></span></code></pre><div class="line-numbers-wrapper" aria-hidden="true"><span class="line-number">1</span><br><span class="line-number">2</span><br><span class="line-number">3</span><br><span class="line-number">4</span><br><span class="line-number">5</span><br><span class="line-number">6</span><br><span class="line-number">7</span><br><span class="line-number">8</span><br><span class="line-number">9</span><br><span class="line-number">10</span><br><span class="line-number">11</span><br><span class="line-number">12</span><br><span class="line-number">13</span><br><span class="line-number">14</span><br><span class="line-number">15</span><br><span class="line-number">16</span><br><span class="line-number">17</span><br><span class="line-number">18</span><br><span class="line-number">19</span><br><span class="line-number">20</span><br><span class="line-number">21</span><br><span class="line-number">22</span><br><span class="line-number">23</span><br><span class="line-number">24</span><br></div></div><h3 id="record-vergleiche" tabindex="-1">Record-Vergleiche <a class="header-anchor" href="#record-vergleiche" aria-label="Permalink to &quot;Record-Vergleiche&quot;">​</a></h3><div class="language-hyp vp-adaptive-theme line-numbers-mode"><button title="Copy Code" class="copy"></button><span class="lang">hyp</span><pre class="shiki shiki-themes github-light github-dark vp-code" tabindex="0"><code><span class="line"><span>Focus {</span></span>
<span class="line"><span>    entrance {</span></span>
<span class="line"><span>        record Vector {</span></span>
<span class="line"><span>            x: number;</span></span>
<span class="line"><span>            y: number;</span></span>
<span class="line"><span>        }</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        induce v1 = Vector { x: 1, y: 2 };</span></span>
<span class="line"><span>        induce v2 = Vector { x: 1, y: 2 };</span></span>
<span class="line"><span>        induce v3 = Vector { x: 3, y: 4 };</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        // Strukturelle Gleichheit</span></span>
<span class="line"><span>        observe &quot;v1 == v2: &quot; + (v1 == v2);  // true</span></span>
<span class="line"><span>        observe &quot;v1 == v3: &quot; + (v1 == v3);  // false</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        // Tiefenvergleich</span></span>
<span class="line"><span>        induce areEqual = DeepEquals(v1, v2);</span></span>
<span class="line"><span>        observe &quot;Tiefenvergleich v1 und v2: &quot; + areEqual;</span></span>
<span class="line"><span>    }</span></span>
<span class="line"><span>} Relax;</span></span></code></pre><div class="line-numbers-wrapper" aria-hidden="true"><span class="line-number">1</span><br><span class="line-number">2</span><br><span class="line-number">3</span><br><span class="line-number">4</span><br><span class="line-number">5</span><br><span class="line-number">6</span><br><span class="line-number">7</span><br><span class="line-number">8</span><br><span class="line-number">9</span><br><span class="line-number">10</span><br><span class="line-number">11</span><br><span class="line-number">12</span><br><span class="line-number">13</span><br><span class="line-number">14</span><br><span class="line-number">15</span><br><span class="line-number">16</span><br><span class="line-number">17</span><br><span class="line-number">18</span><br><span class="line-number">19</span><br><span class="line-number">20</span><br></div></div><h2 id="erweiterte-record-features" tabindex="-1">Erweiterte Record-Features <a class="header-anchor" href="#erweiterte-record-features" aria-label="Permalink to &quot;Erweiterte Record-Features&quot;">​</a></h2><h3 id="record-mit-methoden" tabindex="-1">Record mit Methoden <a class="header-anchor" href="#record-mit-methoden" aria-label="Permalink to &quot;Record mit Methoden&quot;">​</a></h3><div class="language-hyp vp-adaptive-theme line-numbers-mode"><button title="Copy Code" class="copy"></button><span class="lang">hyp</span><pre class="shiki shiki-themes github-light github-dark vp-code" tabindex="0"><code><span class="line"><span>Focus {</span></span>
<span class="line"><span>    entrance {</span></span>
<span class="line"><span>        record Rectangle {</span></span>
<span class="line"><span>            width: number;</span></span>
<span class="line"><span>            height: number;</span></span>
<span class="line"><span></span></span>
<span class="line"><span>            // Methoden im Record</span></span>
<span class="line"><span>            suggestion area(): number {</span></span>
<span class="line"><span>                awaken this.width * this.height;</span></span>
<span class="line"><span>            }</span></span>
<span class="line"><span></span></span>
<span class="line"><span>            suggestion perimeter(): number {</span></span>
<span class="line"><span>                awaken 2 * (this.width + this.height);</span></span>
<span class="line"><span>            }</span></span>
<span class="line"><span></span></span>
<span class="line"><span>            suggestion isSquare(): boolean {</span></span>
<span class="line"><span>                awaken this.width == this.height;</span></span>
<span class="line"><span>            }</span></span>
<span class="line"><span>        }</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        induce rect = Rectangle {</span></span>
<span class="line"><span>            width: 10,</span></span>
<span class="line"><span>            height: 5</span></span>
<span class="line"><span>        };</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        observe &quot;Fläche: &quot; + rect.area();</span></span>
<span class="line"><span>        observe &quot;Umfang: &quot; + rect.perimeter();</span></span>
<span class="line"><span>        observe &quot;Ist Quadrat: &quot; + rect.isSquare();</span></span>
<span class="line"><span>    }</span></span>
<span class="line"><span>} Relax;</span></span></code></pre><div class="line-numbers-wrapper" aria-hidden="true"><span class="line-number">1</span><br><span class="line-number">2</span><br><span class="line-number">3</span><br><span class="line-number">4</span><br><span class="line-number">5</span><br><span class="line-number">6</span><br><span class="line-number">7</span><br><span class="line-number">8</span><br><span class="line-number">9</span><br><span class="line-number">10</span><br><span class="line-number">11</span><br><span class="line-number">12</span><br><span class="line-number">13</span><br><span class="line-number">14</span><br><span class="line-number">15</span><br><span class="line-number">16</span><br><span class="line-number">17</span><br><span class="line-number">18</span><br><span class="line-number">19</span><br><span class="line-number">20</span><br><span class="line-number">21</span><br><span class="line-number">22</span><br><span class="line-number">23</span><br><span class="line-number">24</span><br><span class="line-number">25</span><br><span class="line-number">26</span><br><span class="line-number">27</span><br><span class="line-number">28</span><br><span class="line-number">29</span><br><span class="line-number">30</span><br></div></div><h3 id="record-mit-berechneten-feldern" tabindex="-1">Record mit berechneten Feldern <a class="header-anchor" href="#record-mit-berechneten-feldern" aria-label="Permalink to &quot;Record mit berechneten Feldern&quot;">​</a></h3><div class="language-hyp vp-adaptive-theme line-numbers-mode"><button title="Copy Code" class="copy"></button><span class="lang">hyp</span><pre class="shiki shiki-themes github-light github-dark vp-code" tabindex="0"><code><span class="line"><span>Focus {</span></span>
<span class="line"><span>    entrance {</span></span>
<span class="line"><span>        record Circle {</span></span>
<span class="line"><span>            radius: number;</span></span>
<span class="line"><span>            diameter: number;  // Berechnet aus radius</span></span>
<span class="line"><span></span></span>
<span class="line"><span>            suggestion constructor(r: number) {</span></span>
<span class="line"><span>                this.radius = r;</span></span>
<span class="line"><span>                this.diameter = 2 * r;</span></span>
<span class="line"><span>            }</span></span>
<span class="line"><span>        }</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        induce circle = Circle(5);</span></span>
<span class="line"><span>        observe &quot;Radius: &quot; + circle.radius;</span></span>
<span class="line"><span>        observe &quot;Durchmesser: &quot; + circle.diameter;</span></span>
<span class="line"><span>    }</span></span>
<span class="line"><span>} Relax;</span></span></code></pre><div class="line-numbers-wrapper" aria-hidden="true"><span class="line-number">1</span><br><span class="line-number">2</span><br><span class="line-number">3</span><br><span class="line-number">4</span><br><span class="line-number">5</span><br><span class="line-number">6</span><br><span class="line-number">7</span><br><span class="line-number">8</span><br><span class="line-number">9</span><br><span class="line-number">10</span><br><span class="line-number">11</span><br><span class="line-number">12</span><br><span class="line-number">13</span><br><span class="line-number">14</span><br><span class="line-number">15</span><br><span class="line-number">16</span><br><span class="line-number">17</span><br></div></div><h3 id="record-mit-validierung" tabindex="-1">Record mit Validierung <a class="header-anchor" href="#record-mit-validierung" aria-label="Permalink to &quot;Record mit Validierung&quot;">​</a></h3><div class="language-hyp vp-adaptive-theme line-numbers-mode"><button title="Copy Code" class="copy"></button><span class="lang">hyp</span><pre class="shiki shiki-themes github-light github-dark vp-code" tabindex="0"><code><span class="line"><span>Focus {</span></span>
<span class="line"><span>    entrance {</span></span>
<span class="line"><span>        record Email {</span></span>
<span class="line"><span>            address: string;</span></span>
<span class="line"><span></span></span>
<span class="line"><span>            suggestion constructor(email: string) {</span></span>
<span class="line"><span>                if (IsValidEmail(email)) {</span></span>
<span class="line"><span>                    this.address = email;</span></span>
<span class="line"><span>                } else {</span></span>
<span class="line"><span>                    throw &quot;Ungültige E-Mail-Adresse: &quot; + email;</span></span>
<span class="line"><span>                }</span></span>
<span class="line"><span>            }</span></span>
<span class="line"><span></span></span>
<span class="line"><span>            suggestion getDomain(): string {</span></span>
<span class="line"><span>                induce parts = Split(this.address, &quot;@&quot;);</span></span>
<span class="line"><span>                if (ArrayLength(parts) == 2) {</span></span>
<span class="line"><span>                    awaken parts[1];</span></span>
<span class="line"><span>                } else {</span></span>
<span class="line"><span>                    awaken &quot;&quot;;</span></span>
<span class="line"><span>                }</span></span>
<span class="line"><span>            }</span></span>
<span class="line"><span>        }</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        try {</span></span>
<span class="line"><span>            induce email = Email(&quot;user@example.com&quot;);</span></span>
<span class="line"><span>            observe &quot;E-Mail: &quot; + email.address;</span></span>
<span class="line"><span>            observe &quot;Domain: &quot; + email.getDomain();</span></span>
<span class="line"><span>        } catch (error) {</span></span>
<span class="line"><span>            observe &quot;Fehler: &quot; + error;</span></span>
<span class="line"><span>        }</span></span>
<span class="line"><span>    }</span></span>
<span class="line"><span>} Relax;</span></span></code></pre><div class="line-numbers-wrapper" aria-hidden="true"><span class="line-number">1</span><br><span class="line-number">2</span><br><span class="line-number">3</span><br><span class="line-number">4</span><br><span class="line-number">5</span><br><span class="line-number">6</span><br><span class="line-number">7</span><br><span class="line-number">8</span><br><span class="line-number">9</span><br><span class="line-number">10</span><br><span class="line-number">11</span><br><span class="line-number">12</span><br><span class="line-number">13</span><br><span class="line-number">14</span><br><span class="line-number">15</span><br><span class="line-number">16</span><br><span class="line-number">17</span><br><span class="line-number">18</span><br><span class="line-number">19</span><br><span class="line-number">20</span><br><span class="line-number">21</span><br><span class="line-number">22</span><br><span class="line-number">23</span><br><span class="line-number">24</span><br><span class="line-number">25</span><br><span class="line-number">26</span><br><span class="line-number">27</span><br><span class="line-number">28</span><br><span class="line-number">29</span><br><span class="line-number">30</span><br><span class="line-number">31</span><br><span class="line-number">32</span><br></div></div><h2 id="record-patterns" tabindex="-1">Record-Patterns <a class="header-anchor" href="#record-patterns" aria-label="Permalink to &quot;Record-Patterns&quot;">​</a></h2><h3 id="record-als-konfiguration" tabindex="-1">Record als Konfiguration <a class="header-anchor" href="#record-als-konfiguration" aria-label="Permalink to &quot;Record als Konfiguration&quot;">​</a></h3><div class="language-hyp vp-adaptive-theme line-numbers-mode"><button title="Copy Code" class="copy"></button><span class="lang">hyp</span><pre class="shiki shiki-themes github-light github-dark vp-code" tabindex="0"><code><span class="line"><span>Focus {</span></span>
<span class="line"><span>    entrance {</span></span>
<span class="line"><span>        record DatabaseConfig {</span></span>
<span class="line"><span>            host: string;</span></span>
<span class="line"><span>            port: number;</span></span>
<span class="line"><span>            username: string;</span></span>
<span class="line"><span>            password: string;</span></span>
<span class="line"><span>            database: string;</span></span>
<span class="line"><span>            ssl: boolean;</span></span>
<span class="line"><span>            timeout: number;</span></span>
<span class="line"><span>        }</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        induce dbConfig = DatabaseConfig {</span></span>
<span class="line"><span>            host: &quot;localhost&quot;,</span></span>
<span class="line"><span>            port: 5432,</span></span>
<span class="line"><span>            username: &quot;admin&quot;,</span></span>
<span class="line"><span>            password: &quot;secret123&quot;,</span></span>
<span class="line"><span>            database: &quot;hypnoscript&quot;,</span></span>
<span class="line"><span>            ssl: true,</span></span>
<span class="line"><span>            timeout: 30</span></span>
<span class="line"><span>        };</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        // Konfiguration verwenden</span></span>
<span class="line"><span>        induce connectionString = &quot;postgresql://&quot; + dbConfig.username + &quot;:&quot; +</span></span>
<span class="line"><span>                                 dbConfig.password + &quot;@&quot; + dbConfig.host + &quot;:&quot; +</span></span>
<span class="line"><span>                                 dbConfig.port + &quot;/&quot; + dbConfig.database;</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        observe &quot;Verbindungsstring: &quot; + connectionString;</span></span>
<span class="line"><span>    }</span></span>
<span class="line"><span>} Relax;</span></span></code></pre><div class="line-numbers-wrapper" aria-hidden="true"><span class="line-number">1</span><br><span class="line-number">2</span><br><span class="line-number">3</span><br><span class="line-number">4</span><br><span class="line-number">5</span><br><span class="line-number">6</span><br><span class="line-number">7</span><br><span class="line-number">8</span><br><span class="line-number">9</span><br><span class="line-number">10</span><br><span class="line-number">11</span><br><span class="line-number">12</span><br><span class="line-number">13</span><br><span class="line-number">14</span><br><span class="line-number">15</span><br><span class="line-number">16</span><br><span class="line-number">17</span><br><span class="line-number">18</span><br><span class="line-number">19</span><br><span class="line-number">20</span><br><span class="line-number">21</span><br><span class="line-number">22</span><br><span class="line-number">23</span><br><span class="line-number">24</span><br><span class="line-number">25</span><br><span class="line-number">26</span><br><span class="line-number">27</span><br><span class="line-number">28</span><br><span class="line-number">29</span><br><span class="line-number">30</span><br></div></div><h3 id="record-als-api-response" tabindex="-1">Record als API-Response <a class="header-anchor" href="#record-als-api-response" aria-label="Permalink to &quot;Record als API-Response&quot;">​</a></h3><div class="language-hyp vp-adaptive-theme line-numbers-mode"><button title="Copy Code" class="copy"></button><span class="lang">hyp</span><pre class="shiki shiki-themes github-light github-dark vp-code" tabindex="0"><code><span class="line"><span>Focus {</span></span>
<span class="line"><span>    entrance {</span></span>
<span class="line"><span>        record ApiResponse {</span></span>
<span class="line"><span>            success: boolean;</span></span>
<span class="line"><span>            data?: object;</span></span>
<span class="line"><span>            error?: string;</span></span>
<span class="line"><span>            timestamp: number;</span></span>
<span class="line"><span>            requestId: string;</span></span>
<span class="line"><span>        }</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        // Erfolgreiche Antwort</span></span>
<span class="line"><span>        induce successResponse = ApiResponse {</span></span>
<span class="line"><span>            success: true,</span></span>
<span class="line"><span>            data: {</span></span>
<span class="line"><span>                userId: 123,</span></span>
<span class="line"><span>                name: &quot;Alice&quot;,</span></span>
<span class="line"><span>                email: &quot;alice@example.com&quot;</span></span>
<span class="line"><span>            },</span></span>
<span class="line"><span>            timestamp: GetCurrentTime(),</span></span>
<span class="line"><span>            requestId: GenerateUUID()</span></span>
<span class="line"><span>        };</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        // Fehlerantwort</span></span>
<span class="line"><span>        induce errorResponse = ApiResponse {</span></span>
<span class="line"><span>            success: false,</span></span>
<span class="line"><span>            error: &quot;Benutzer nicht gefunden&quot;,</span></span>
<span class="line"><span>            timestamp: GetCurrentTime(),</span></span>
<span class="line"><span>            requestId: GenerateUUID()</span></span>
<span class="line"><span>        };</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        observe &quot;Erfolg: &quot; + successResponse.success;</span></span>
<span class="line"><span>        observe &quot;Fehler: &quot; + errorResponse.error;</span></span>
<span class="line"><span>    }</span></span>
<span class="line"><span>} Relax;</span></span></code></pre><div class="line-numbers-wrapper" aria-hidden="true"><span class="line-number">1</span><br><span class="line-number">2</span><br><span class="line-number">3</span><br><span class="line-number">4</span><br><span class="line-number">5</span><br><span class="line-number">6</span><br><span class="line-number">7</span><br><span class="line-number">8</span><br><span class="line-number">9</span><br><span class="line-number">10</span><br><span class="line-number">11</span><br><span class="line-number">12</span><br><span class="line-number">13</span><br><span class="line-number">14</span><br><span class="line-number">15</span><br><span class="line-number">16</span><br><span class="line-number">17</span><br><span class="line-number">18</span><br><span class="line-number">19</span><br><span class="line-number">20</span><br><span class="line-number">21</span><br><span class="line-number">22</span><br><span class="line-number">23</span><br><span class="line-number">24</span><br><span class="line-number">25</span><br><span class="line-number">26</span><br><span class="line-number">27</span><br><span class="line-number">28</span><br><span class="line-number">29</span><br><span class="line-number">30</span><br><span class="line-number">31</span><br><span class="line-number">32</span><br><span class="line-number">33</span><br><span class="line-number">34</span><br></div></div><h3 id="record-fur-event-handling" tabindex="-1">Record für Event-Handling <a class="header-anchor" href="#record-fur-event-handling" aria-label="Permalink to &quot;Record für Event-Handling&quot;">​</a></h3><div class="language-hyp vp-adaptive-theme line-numbers-mode"><button title="Copy Code" class="copy"></button><span class="lang">hyp</span><pre class="shiki shiki-themes github-light github-dark vp-code" tabindex="0"><code><span class="line"><span>Focus {</span></span>
<span class="line"><span>    entrance {</span></span>
<span class="line"><span>        record Event {</span></span>
<span class="line"><span>            type: string;</span></span>
<span class="line"><span>            source: string;</span></span>
<span class="line"><span>            timestamp: number;</span></span>
<span class="line"><span>            data: object;</span></span>
<span class="line"><span>            priority: number;</span></span>
<span class="line"><span>        }</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        induce userEvent = Event {</span></span>
<span class="line"><span>            type: &quot;user.login&quot;,</span></span>
<span class="line"><span>            source: &quot;web-interface&quot;,</span></span>
<span class="line"><span>            timestamp: GetCurrentTime(),</span></span>
<span class="line"><span>            data: {</span></span>
<span class="line"><span>                userId: 456,</span></span>
<span class="line"><span>                ipAddress: &quot;192.168.1.100&quot;,</span></span>
<span class="line"><span>                userAgent: &quot;Mozilla/5.0...&quot;</span></span>
<span class="line"><span>            },</span></span>
<span class="line"><span>            priority: 1</span></span>
<span class="line"><span>        };</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        // Event verarbeiten</span></span>
<span class="line"><span>        if (userEvent.type == &quot;user.login&quot;) {</span></span>
<span class="line"><span>            observe &quot;Benutzer-Login erkannt: &quot; + userEvent.data.userId;</span></span>
<span class="line"><span>            LogEvent(userEvent);</span></span>
<span class="line"><span>        }</span></span>
<span class="line"><span>    }</span></span>
<span class="line"><span>} Relax;</span></span></code></pre><div class="line-numbers-wrapper" aria-hidden="true"><span class="line-number">1</span><br><span class="line-number">2</span><br><span class="line-number">3</span><br><span class="line-number">4</span><br><span class="line-number">5</span><br><span class="line-number">6</span><br><span class="line-number">7</span><br><span class="line-number">8</span><br><span class="line-number">9</span><br><span class="line-number">10</span><br><span class="line-number">11</span><br><span class="line-number">12</span><br><span class="line-number">13</span><br><span class="line-number">14</span><br><span class="line-number">15</span><br><span class="line-number">16</span><br><span class="line-number">17</span><br><span class="line-number">18</span><br><span class="line-number">19</span><br><span class="line-number">20</span><br><span class="line-number">21</span><br><span class="line-number">22</span><br><span class="line-number">23</span><br><span class="line-number">24</span><br><span class="line-number">25</span><br><span class="line-number">26</span><br><span class="line-number">27</span><br><span class="line-number">28</span><br><span class="line-number">29</span><br></div></div><h2 id="record-arrays-und-collections" tabindex="-1">Record-Arrays und Collections <a class="header-anchor" href="#record-arrays-und-collections" aria-label="Permalink to &quot;Record-Arrays und Collections&quot;">​</a></h2><h3 id="array-von-records" tabindex="-1">Array von Records <a class="header-anchor" href="#array-von-records" aria-label="Permalink to &quot;Array von Records&quot;">​</a></h3><div class="language-hyp vp-adaptive-theme line-numbers-mode"><button title="Copy Code" class="copy"></button><span class="lang">hyp</span><pre class="shiki shiki-themes github-light github-dark vp-code" tabindex="0"><code><span class="line"><span>Focus {</span></span>
<span class="line"><span>    entrance {</span></span>
<span class="line"><span>        record Student {</span></span>
<span class="line"><span>            id: number;</span></span>
<span class="line"><span>            name: string;</span></span>
<span class="line"><span>            grade: number;</span></span>
<span class="line"><span>        }</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        induce students = [</span></span>
<span class="line"><span>            Student { id: 1, name: &quot;Alice&quot;, grade: 85 },</span></span>
<span class="line"><span>            Student { id: 2, name: &quot;Bob&quot;, grade: 92 },</span></span>
<span class="line"><span>            Student { id: 3, name: &quot;Charlie&quot;, grade: 78 }</span></span>
<span class="line"><span>        ];</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        // Durch Records iterieren</span></span>
<span class="line"><span>        for (induce i = 0; i &lt; ArrayLength(students); induce i = i + 1) {</span></span>
<span class="line"><span>            induce student = students[i];</span></span>
<span class="line"><span>            observe &quot;Student: &quot; + student.name + &quot; - Note: &quot; + student.grade;</span></span>
<span class="line"><span>        }</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        // Records filtern</span></span>
<span class="line"><span>        induce topStudents = ArrayFilter(students, function(student) {</span></span>
<span class="line"><span>            return student.grade &gt;= 90;</span></span>
<span class="line"><span>        });</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        observe &quot;Top-Studenten: &quot; + ArrayLength(topStudents);</span></span>
<span class="line"><span>    }</span></span>
<span class="line"><span>} Relax;</span></span></code></pre><div class="line-numbers-wrapper" aria-hidden="true"><span class="line-number">1</span><br><span class="line-number">2</span><br><span class="line-number">3</span><br><span class="line-number">4</span><br><span class="line-number">5</span><br><span class="line-number">6</span><br><span class="line-number">7</span><br><span class="line-number">8</span><br><span class="line-number">9</span><br><span class="line-number">10</span><br><span class="line-number">11</span><br><span class="line-number">12</span><br><span class="line-number">13</span><br><span class="line-number">14</span><br><span class="line-number">15</span><br><span class="line-number">16</span><br><span class="line-number">17</span><br><span class="line-number">18</span><br><span class="line-number">19</span><br><span class="line-number">20</span><br><span class="line-number">21</span><br><span class="line-number">22</span><br><span class="line-number">23</span><br><span class="line-number">24</span><br><span class="line-number">25</span><br><span class="line-number">26</span><br><span class="line-number">27</span><br><span class="line-number">28</span><br></div></div><h3 id="record-als-dictionary-wert" tabindex="-1">Record als Dictionary-Wert <a class="header-anchor" href="#record-als-dictionary-wert" aria-label="Permalink to &quot;Record als Dictionary-Wert&quot;">​</a></h3><div class="language-hyp vp-adaptive-theme line-numbers-mode"><button title="Copy Code" class="copy"></button><span class="lang">hyp</span><pre class="shiki shiki-themes github-light github-dark vp-code" tabindex="0"><code><span class="line"><span>Focus {</span></span>
<span class="line"><span>    entrance {</span></span>
<span class="line"><span>        record ProductInfo {</span></span>
<span class="line"><span>            name: string;</span></span>
<span class="line"><span>            price: number;</span></span>
<span class="line"><span>            category: string;</span></span>
<span class="line"><span>        }</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        induce productCatalog = {</span></span>
<span class="line"><span>            &quot;PROD001&quot;: ProductInfo { name: &quot;Laptop&quot;, price: 999.99, category: &quot;Electronics&quot; },</span></span>
<span class="line"><span>            &quot;PROD002&quot;: ProductInfo { name: &quot;Mouse&quot;, price: 29.99, category: &quot;Electronics&quot; },</span></span>
<span class="line"><span>            &quot;PROD003&quot;: ProductInfo { name: &quot;Book&quot;, price: 19.99, category: &quot;Books&quot; }</span></span>
<span class="line"><span>        };</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        // Produkt nach ID suchen</span></span>
<span class="line"><span>        induce productId = &quot;PROD001&quot;;</span></span>
<span class="line"><span>        if (productCatalog[productId]) {</span></span>
<span class="line"><span>            induce product = productCatalog[productId];</span></span>
<span class="line"><span>            observe &quot;Produkt gefunden: &quot; + product.name + &quot; - &quot; + product.price + &quot; €&quot;;</span></span>
<span class="line"><span>        }</span></span>
<span class="line"><span>    }</span></span>
<span class="line"><span>} Relax;</span></span></code></pre><div class="line-numbers-wrapper" aria-hidden="true"><span class="line-number">1</span><br><span class="line-number">2</span><br><span class="line-number">3</span><br><span class="line-number">4</span><br><span class="line-number">5</span><br><span class="line-number">6</span><br><span class="line-number">7</span><br><span class="line-number">8</span><br><span class="line-number">9</span><br><span class="line-number">10</span><br><span class="line-number">11</span><br><span class="line-number">12</span><br><span class="line-number">13</span><br><span class="line-number">14</span><br><span class="line-number">15</span><br><span class="line-number">16</span><br><span class="line-number">17</span><br><span class="line-number">18</span><br><span class="line-number">19</span><br><span class="line-number">20</span><br><span class="line-number">21</span><br><span class="line-number">22</span><br></div></div><h2 id="best-practices" tabindex="-1">Best Practices <a class="header-anchor" href="#best-practices" aria-label="Permalink to &quot;Best Practices&quot;">​</a></h2><h3 id="record-design" tabindex="-1">Record-Design <a class="header-anchor" href="#record-design" aria-label="Permalink to &quot;Record-Design&quot;">​</a></h3><div class="language-hyp vp-adaptive-theme line-numbers-mode"><button title="Copy Code" class="copy"></button><span class="lang">hyp</span><pre class="shiki shiki-themes github-light github-dark vp-code" tabindex="0"><code><span class="line"><span>Focus {</span></span>
<span class="line"><span>    entrance {</span></span>
<span class="line"><span>        // ✅ GUT: Klare, spezifische Records</span></span>
<span class="line"><span>        record UserProfile {</span></span>
<span class="line"><span>            userId: number;</span></span>
<span class="line"><span>            displayName: string;</span></span>
<span class="line"><span>            email: string;</span></span>
<span class="line"><span>            preferences: object;</span></span>
<span class="line"><span>        }</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        // ❌ SCHLECHT: Zu generische Records</span></span>
<span class="line"><span>        record Data {</span></span>
<span class="line"><span>            field1: object;</span></span>
<span class="line"><span>            field2: object;</span></span>
<span class="line"><span>            field3: object;</span></span>
<span class="line"><span>        }</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        // ✅ GUT: Immutable Records verwenden</span></span>
<span class="line"><span>        induce user = UserProfile {</span></span>
<span class="line"><span>            userId: 123,</span></span>
<span class="line"><span>            displayName: &quot;Alice&quot;,</span></span>
<span class="line"><span>            email: &quot;alice@example.com&quot;,</span></span>
<span class="line"><span>            preferences: {</span></span>
<span class="line"><span>                theme: &quot;dark&quot;,</span></span>
<span class="line"><span>                language: &quot;de&quot;</span></span>
<span class="line"><span>            }</span></span>
<span class="line"><span>        };</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        // ✅ GUT: Kopien für Änderungen erstellen</span></span>
<span class="line"><span>        induce updatedUser = user with {</span></span>
<span class="line"><span>            displayName: &quot;Alice Johnson&quot;</span></span>
<span class="line"><span>        };</span></span>
<span class="line"><span>    }</span></span>
<span class="line"><span>} Relax;</span></span></code></pre><div class="line-numbers-wrapper" aria-hidden="true"><span class="line-number">1</span><br><span class="line-number">2</span><br><span class="line-number">3</span><br><span class="line-number">4</span><br><span class="line-number">5</span><br><span class="line-number">6</span><br><span class="line-number">7</span><br><span class="line-number">8</span><br><span class="line-number">9</span><br><span class="line-number">10</span><br><span class="line-number">11</span><br><span class="line-number">12</span><br><span class="line-number">13</span><br><span class="line-number">14</span><br><span class="line-number">15</span><br><span class="line-number">16</span><br><span class="line-number">17</span><br><span class="line-number">18</span><br><span class="line-number">19</span><br><span class="line-number">20</span><br><span class="line-number">21</span><br><span class="line-number">22</span><br><span class="line-number">23</span><br><span class="line-number">24</span><br><span class="line-number">25</span><br><span class="line-number">26</span><br><span class="line-number">27</span><br><span class="line-number">28</span><br><span class="line-number">29</span><br><span class="line-number">30</span><br><span class="line-number">31</span><br><span class="line-number">32</span><br><span class="line-number">33</span><br><span class="line-number">34</span><br></div></div><h3 id="performance-optimierung" tabindex="-1">Performance-Optimierung <a class="header-anchor" href="#performance-optimierung" aria-label="Permalink to &quot;Performance-Optimierung&quot;">​</a></h3><div class="language-hyp vp-adaptive-theme line-numbers-mode"><button title="Copy Code" class="copy"></button><span class="lang">hyp</span><pre class="shiki shiki-themes github-light github-dark vp-code" tabindex="0"><code><span class="line"><span>Focus {</span></span>
<span class="line"><span>    entrance {</span></span>
<span class="line"><span>        // ✅ GUT: Records für kleine, häufig verwendete Daten</span></span>
<span class="line"><span>        record Point {</span></span>
<span class="line"><span>            x: number;</span></span>
<span class="line"><span>            y: number;</span></span>
<span class="line"><span>        }</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        // ✅ GUT: Sessions für komplexe Objekte mit Verhalten</span></span>
<span class="line"><span>        session ComplexObject {</span></span>
<span class="line"><span>            expose data: object;</span></span>
<span class="line"><span></span></span>
<span class="line"><span>            suggestion processData() {</span></span>
<span class="line"><span>                // Komplexe Verarbeitung</span></span>
<span class="line"><span>            }</span></span>
<span class="line"><span>        }</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        // ✅ GUT: Records für Konfigurationen</span></span>
<span class="line"><span>        record AppConfig {</span></span>
<span class="line"><span>            debug: boolean;</span></span>
<span class="line"><span>            logLevel: string;</span></span>
<span class="line"><span>            maxConnections: number;</span></span>
<span class="line"><span>        }</span></span>
<span class="line"><span>    }</span></span>
<span class="line"><span>} Relax;</span></span></code></pre><div class="line-numbers-wrapper" aria-hidden="true"><span class="line-number">1</span><br><span class="line-number">2</span><br><span class="line-number">3</span><br><span class="line-number">4</span><br><span class="line-number">5</span><br><span class="line-number">6</span><br><span class="line-number">7</span><br><span class="line-number">8</span><br><span class="line-number">9</span><br><span class="line-number">10</span><br><span class="line-number">11</span><br><span class="line-number">12</span><br><span class="line-number">13</span><br><span class="line-number">14</span><br><span class="line-number">15</span><br><span class="line-number">16</span><br><span class="line-number">17</span><br><span class="line-number">18</span><br><span class="line-number">19</span><br><span class="line-number">20</span><br><span class="line-number">21</span><br><span class="line-number">22</span><br><span class="line-number">23</span><br><span class="line-number">24</span><br><span class="line-number">25</span><br></div></div><h3 id="fehlerbehandlung" tabindex="-1">Fehlerbehandlung <a class="header-anchor" href="#fehlerbehandlung" aria-label="Permalink to &quot;Fehlerbehandlung&quot;">​</a></h3><div class="language-hyp vp-adaptive-theme line-numbers-mode"><button title="Copy Code" class="copy"></button><span class="lang">hyp</span><pre class="shiki shiki-themes github-light github-dark vp-code" tabindex="0"><code><span class="line"><span>Focus {</span></span>
<span class="line"><span>    entrance {</span></span>
<span class="line"><span>        record ValidationResult {</span></span>
<span class="line"><span>            isValid: boolean;</span></span>
<span class="line"><span>            errors: array;</span></span>
<span class="line"><span>            warnings: array;</span></span>
<span class="line"><span>        }</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        suggestion validateEmail(email: string): ValidationResult {</span></span>
<span class="line"><span>            induce errors = [];</span></span>
<span class="line"><span>            induce warnings = [];</span></span>
<span class="line"><span></span></span>
<span class="line"><span>            if (Length(email) == 0) {</span></span>
<span class="line"><span>                ArrayPush(errors, &quot;E-Mail darf nicht leer sein&quot;);</span></span>
<span class="line"><span>            } else if (!IsValidEmail(email)) {</span></span>
<span class="line"><span>                ArrayPush(errors, &quot;Ungültiges E-Mail-Format&quot;);</span></span>
<span class="line"><span>            }</span></span>
<span class="line"><span></span></span>
<span class="line"><span>            if (Length(email) &gt; 100) {</span></span>
<span class="line"><span>                ArrayPush(warnings, &quot;E-Mail ist sehr lang&quot;);</span></span>
<span class="line"><span>            }</span></span>
<span class="line"><span></span></span>
<span class="line"><span>            return ValidationResult {</span></span>
<span class="line"><span>                isValid: ArrayLength(errors) == 0,</span></span>
<span class="line"><span>                errors: errors,</span></span>
<span class="line"><span>                warnings: warnings</span></span>
<span class="line"><span>            };</span></span>
<span class="line"><span>        }</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        induce result = validateEmail(&quot;test@example.com&quot;);</span></span>
<span class="line"><span>        if (result.isValid) {</span></span>
<span class="line"><span>            observe &quot;E-Mail ist gültig&quot;;</span></span>
<span class="line"><span>        } else {</span></span>
<span class="line"><span>            observe &quot;E-Mail-Fehler: &quot; + result.errors;</span></span>
<span class="line"><span>        }</span></span>
<span class="line"><span>    }</span></span>
<span class="line"><span>} Relax;</span></span></code></pre><div class="line-numbers-wrapper" aria-hidden="true"><span class="line-number">1</span><br><span class="line-number">2</span><br><span class="line-number">3</span><br><span class="line-number">4</span><br><span class="line-number">5</span><br><span class="line-number">6</span><br><span class="line-number">7</span><br><span class="line-number">8</span><br><span class="line-number">9</span><br><span class="line-number">10</span><br><span class="line-number">11</span><br><span class="line-number">12</span><br><span class="line-number">13</span><br><span class="line-number">14</span><br><span class="line-number">15</span><br><span class="line-number">16</span><br><span class="line-number">17</span><br><span class="line-number">18</span><br><span class="line-number">19</span><br><span class="line-number">20</span><br><span class="line-number">21</span><br><span class="line-number">22</span><br><span class="line-number">23</span><br><span class="line-number">24</span><br><span class="line-number">25</span><br><span class="line-number">26</span><br><span class="line-number">27</span><br><span class="line-number">28</span><br><span class="line-number">29</span><br><span class="line-number">30</span><br><span class="line-number">31</span><br><span class="line-number">32</span><br><span class="line-number">33</span><br><span class="line-number">34</span><br><span class="line-number">35</span><br><span class="line-number">36</span><br><span class="line-number">37</span><br></div></div><h2 id="fehlerbehandlung-1" tabindex="-1">Fehlerbehandlung <a class="header-anchor" href="#fehlerbehandlung-1" aria-label="Permalink to &quot;Fehlerbehandlung&quot;">​</a></h2><p>Records können bei ungültigen Operationen Fehler werfen:</p><div class="language-hyp vp-adaptive-theme line-numbers-mode"><button title="Copy Code" class="copy"></button><span class="lang">hyp</span><pre class="shiki shiki-themes github-light github-dark vp-code" tabindex="0"><code><span class="line"><span>Focus {</span></span>
<span class="line"><span>    entrance {</span></span>
<span class="line"><span>        try {</span></span>
<span class="line"><span>            record Person {</span></span>
<span class="line"><span>                name: string;</span></span>
<span class="line"><span>                age: number;</span></span>
<span class="line"><span>            }</span></span>
<span class="line"><span></span></span>
<span class="line"><span>            induce person = Person {</span></span>
<span class="line"><span>                name: &quot;Alice&quot;,</span></span>
<span class="line"><span>                age: 30</span></span>
<span class="line"><span>            };</span></span>
<span class="line"><span></span></span>
<span class="line"><span>            // Ungültiger Feldzugriff</span></span>
<span class="line"><span>            induce invalidField = person.nonexistentField;</span></span>
<span class="line"><span>        } catch (error) {</span></span>
<span class="line"><span>            observe &quot;Record-Fehler: &quot; + error;</span></span>
<span class="line"><span>        }</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        try {</span></span>
<span class="line"><span>            // Ungültige Record-Erstellung</span></span>
<span class="line"><span>            induce invalidPerson = Person {</span></span>
<span class="line"><span>                name: &quot;Bob&quot;,</span></span>
<span class="line"><span>                age: &quot;ungültig&quot;  // Sollte number sein</span></span>
<span class="line"><span>            };</span></span>
<span class="line"><span>        } catch (error) {</span></span>
<span class="line"><span>            observe &quot;Validierungsfehler: &quot; + error;</span></span>
<span class="line"><span>        }</span></span>
<span class="line"><span>    }</span></span>
<span class="line"><span>} Relax;</span></span></code></pre><div class="line-numbers-wrapper" aria-hidden="true"><span class="line-number">1</span><br><span class="line-number">2</span><br><span class="line-number">3</span><br><span class="line-number">4</span><br><span class="line-number">5</span><br><span class="line-number">6</span><br><span class="line-number">7</span><br><span class="line-number">8</span><br><span class="line-number">9</span><br><span class="line-number">10</span><br><span class="line-number">11</span><br><span class="line-number">12</span><br><span class="line-number">13</span><br><span class="line-number">14</span><br><span class="line-number">15</span><br><span class="line-number">16</span><br><span class="line-number">17</span><br><span class="line-number">18</span><br><span class="line-number">19</span><br><span class="line-number">20</span><br><span class="line-number">21</span><br><span class="line-number">22</span><br><span class="line-number">23</span><br><span class="line-number">24</span><br><span class="line-number">25</span><br><span class="line-number">26</span><br><span class="line-number">27</span><br><span class="line-number">28</span><br><span class="line-number">29</span><br><span class="line-number">30</span><br></div></div><h2 id="nachste-schritte" tabindex="-1">Nächste Schritte <a class="header-anchor" href="#nachste-schritte" aria-label="Permalink to &quot;Nächste Schritte&quot;">​</a></h2><ul><li><a href="./sessions.html">Sessions</a> - Objektorientierte Programmierung mit Sessions</li><li><a href="./arrays.html">Arrays</a> - Array-Operationen und Collections</li><li><a href="./functions.html">Functions</a> - Funktionsdefinitionen und -aufrufe</li></ul><hr><p><strong>Records gemeistert? Dann lerne <a href="./sessions.html">Sessions</a> kennen!</strong> ✅</p>`,56)])])}const d=s(l,[["render",r]]);export{m as __pageData,d as default};
