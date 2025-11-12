import{_ as s,c as a,o as e,ag as p}from"./chunks/framework.Dli2S8Ej.js";const d=JSON.parse('{"title":"Beispiele: System-Funktionen","description":"","frontmatter":{"sidebar_position":2},"headers":[],"relativePath":"examples/system-examples.md","filePath":"examples/system-examples.md","lastUpdated":1750547232000}'),l={name:"examples/system-examples.md"};function i(t,n,r,u,c,o){return e(),a("div",null,[...n[0]||(n[0]=[p(`<h1 id="beispiele-system-funktionen" tabindex="-1">Beispiele: System-Funktionen <a class="header-anchor" href="#beispiele-system-funktionen" aria-label="Permalink to &quot;Beispiele: System-Funktionen&quot;">​</a></h1><p>Diese Seite zeigt praxisnahe Beispiele für System-Funktionen in HypnoScript. Die Beispiele sind kommentiert und können direkt übernommen oder angepasst werden.</p><h2 id="dateioperationen-lesen-schreiben-backup" tabindex="-1">Dateioperationen: Lesen, Schreiben, Backup <a class="header-anchor" href="#dateioperationen-lesen-schreiben-backup" aria-label="Permalink to &quot;Dateioperationen: Lesen, Schreiben, Backup&quot;">​</a></h2><div class="language-hyp vp-adaptive-theme line-numbers-mode"><button title="Copy Code" class="copy"></button><span class="lang">hyp</span><pre class="shiki shiki-themes github-light github-dark vp-code" tabindex="0"><code><span class="line"><span>Focus {</span></span>
<span class="line"><span>    entrance {</span></span>
<span class="line"><span>        // Datei schreiben</span></span>
<span class="line"><span>        WriteFile(&quot;beispiel.txt&quot;, &quot;Hallo HypnoScript!&quot;);</span></span>
<span class="line"><span>        // Datei lesen</span></span>
<span class="line"><span>        induce content = ReadFile(&quot;beispiel.txt&quot;);</span></span>
<span class="line"><span>        observe &quot;Datei-Inhalt: &quot; + content;</span></span>
<span class="line"><span>        // Backup anlegen</span></span>
<span class="line"><span>        induce backupName = &quot;beispiel_backup_&quot; + Timestamp() + &quot;.txt&quot;;</span></span>
<span class="line"><span>        CopyFile(&quot;beispiel.txt&quot;, backupName);</span></span>
<span class="line"><span>        observe &quot;Backup erstellt: &quot; + backupName;</span></span>
<span class="line"><span>    }</span></span>
<span class="line"><span>} Relax;</span></span></code></pre><div class="line-numbers-wrapper" aria-hidden="true"><span class="line-number">1</span><br><span class="line-number">2</span><br><span class="line-number">3</span><br><span class="line-number">4</span><br><span class="line-number">5</span><br><span class="line-number">6</span><br><span class="line-number">7</span><br><span class="line-number">8</span><br><span class="line-number">9</span><br><span class="line-number">10</span><br><span class="line-number">11</span><br><span class="line-number">12</span><br><span class="line-number">13</span><br></div></div><h2 id="verzeichnisse-und-dateilisten" tabindex="-1">Verzeichnisse und Dateilisten <a class="header-anchor" href="#verzeichnisse-und-dateilisten" aria-label="Permalink to &quot;Verzeichnisse und Dateilisten&quot;">​</a></h2><div class="language-hyp vp-adaptive-theme line-numbers-mode"><button title="Copy Code" class="copy"></button><span class="lang">hyp</span><pre class="shiki shiki-themes github-light github-dark vp-code" tabindex="0"><code><span class="line"><span>Focus {</span></span>
<span class="line"><span>    entrance {</span></span>
<span class="line"><span>        // Verzeichnis anlegen</span></span>
<span class="line"><span>        if (!DirectoryExists(&quot;daten&quot;)) CreateDirectory(&quot;daten&quot;);</span></span>
<span class="line"><span>        // Dateien auflisten</span></span>
<span class="line"><span>        induce files = ListFiles(&quot;.&quot;);</span></span>
<span class="line"><span>        observe &quot;Dateien im aktuellen Verzeichnis: &quot; + files;</span></span>
<span class="line"><span>    }</span></span>
<span class="line"><span>} Relax;</span></span></code></pre><div class="line-numbers-wrapper" aria-hidden="true"><span class="line-number">1</span><br><span class="line-number">2</span><br><span class="line-number">3</span><br><span class="line-number">4</span><br><span class="line-number">5</span><br><span class="line-number">6</span><br><span class="line-number">7</span><br><span class="line-number">8</span><br><span class="line-number">9</span><br></div></div><h2 id="automatisierte-dateiverarbeitung" tabindex="-1">Automatisierte Dateiverarbeitung <a class="header-anchor" href="#automatisierte-dateiverarbeitung" aria-label="Permalink to &quot;Automatisierte Dateiverarbeitung&quot;">​</a></h2><div class="language-hyp vp-adaptive-theme line-numbers-mode"><button title="Copy Code" class="copy"></button><span class="lang">hyp</span><pre class="shiki shiki-themes github-light github-dark vp-code" tabindex="0"><code><span class="line"><span>Focus {</span></span>
<span class="line"><span>    entrance {</span></span>
<span class="line"><span>        induce inputDir = &quot;input&quot;;</span></span>
<span class="line"><span>        induce outputDir = &quot;output&quot;;</span></span>
<span class="line"><span>        if (!DirectoryExists(outputDir)) CreateDirectory(outputDir);</span></span>
<span class="line"><span>        induce files = ListFiles(inputDir);</span></span>
<span class="line"><span>        for (induce i = 0; i &lt; ArrayLength(files); induce i = i + 1) {</span></span>
<span class="line"><span>            induce file = ArrayGet(files, i);</span></span>
<span class="line"><span>            induce content = ReadFile(inputDir + &quot;/&quot; + file);</span></span>
<span class="line"><span>            induce processed = ToUpper(content);</span></span>
<span class="line"><span>            WriteFile(outputDir + &quot;/&quot; + file, processed);</span></span>
<span class="line"><span>            observe &quot;Verarbeitet: &quot; + file;</span></span>
<span class="line"><span>        }</span></span>
<span class="line"><span>    }</span></span>
<span class="line"><span>} Relax;</span></span></code></pre><div class="line-numbers-wrapper" aria-hidden="true"><span class="line-number">1</span><br><span class="line-number">2</span><br><span class="line-number">3</span><br><span class="line-number">4</span><br><span class="line-number">5</span><br><span class="line-number">6</span><br><span class="line-number">7</span><br><span class="line-number">8</span><br><span class="line-number">9</span><br><span class="line-number">10</span><br><span class="line-number">11</span><br><span class="line-number">12</span><br><span class="line-number">13</span><br><span class="line-number">14</span><br><span class="line-number">15</span><br></div></div><h2 id="prozessmanagement-systembefehle-ausfuhren" tabindex="-1">Prozessmanagement: Systembefehle ausführen <a class="header-anchor" href="#prozessmanagement-systembefehle-ausfuhren" aria-label="Permalink to &quot;Prozessmanagement: Systembefehle ausführen&quot;">​</a></h2><div class="language-hyp vp-adaptive-theme line-numbers-mode"><button title="Copy Code" class="copy"></button><span class="lang">hyp</span><pre class="shiki shiki-themes github-light github-dark vp-code" tabindex="0"><code><span class="line"><span>Focus {</span></span>
<span class="line"><span>    entrance {</span></span>
<span class="line"><span>        induce result = ExecuteCommand(&quot;echo Hallo von der Shell!&quot;);</span></span>
<span class="line"><span>        observe &quot;Shell-Ausgabe: &quot; + result;</span></span>
<span class="line"><span>    }</span></span>
<span class="line"><span>} Relax;</span></span></code></pre><div class="line-numbers-wrapper" aria-hidden="true"><span class="line-number">1</span><br><span class="line-number">2</span><br><span class="line-number">3</span><br><span class="line-number">4</span><br><span class="line-number">5</span><br><span class="line-number">6</span><br></div></div><h2 id="umgebungsvariablen-lesen-und-setzen" tabindex="-1">Umgebungsvariablen lesen und setzen <a class="header-anchor" href="#umgebungsvariablen-lesen-und-setzen" aria-label="Permalink to &quot;Umgebungsvariablen lesen und setzen&quot;">​</a></h2><div class="language-hyp vp-adaptive-theme line-numbers-mode"><button title="Copy Code" class="copy"></button><span class="lang">hyp</span><pre class="shiki shiki-themes github-light github-dark vp-code" tabindex="0"><code><span class="line"><span>Focus {</span></span>
<span class="line"><span>    entrance {</span></span>
<span class="line"><span>        SetEnvironmentVariable(&quot;MEIN_VAR&quot;, &quot;Testwert&quot;);</span></span>
<span class="line"><span>        induce value = GetEnvironmentVariable(&quot;MEIN_VAR&quot;);</span></span>
<span class="line"><span>        observe &quot;MEIN_VAR: &quot; + value;</span></span>
<span class="line"><span>    }</span></span>
<span class="line"><span>} Relax;</span></span></code></pre><div class="line-numbers-wrapper" aria-hidden="true"><span class="line-number">1</span><br><span class="line-number">2</span><br><span class="line-number">3</span><br><span class="line-number">4</span><br><span class="line-number">5</span><br><span class="line-number">6</span><br><span class="line-number">7</span><br></div></div><h2 id="systeminformationen-und-monitoring" tabindex="-1">Systeminformationen und Monitoring <a class="header-anchor" href="#systeminformationen-und-monitoring" aria-label="Permalink to &quot;Systeminformationen und Monitoring&quot;">​</a></h2><div class="language-hyp vp-adaptive-theme line-numbers-mode"><button title="Copy Code" class="copy"></button><span class="lang">hyp</span><pre class="shiki shiki-themes github-light github-dark vp-code" tabindex="0"><code><span class="line"><span>Focus {</span></span>
<span class="line"><span>    entrance {</span></span>
<span class="line"><span>        induce sys = GetSystemInfo();</span></span>
<span class="line"><span>        induce mem = GetMemoryInfo();</span></span>
<span class="line"><span>        observe &quot;OS: &quot; + sys.os;</span></span>
<span class="line"><span>        observe &quot;RAM: &quot; + mem.used + &quot;/&quot; + mem.total + &quot; MB verwendet&quot;;</span></span>
<span class="line"><span>    }</span></span>
<span class="line"><span>} Relax;</span></span></code></pre><div class="line-numbers-wrapper" aria-hidden="true"><span class="line-number">1</span><br><span class="line-number">2</span><br><span class="line-number">3</span><br><span class="line-number">4</span><br><span class="line-number">5</span><br><span class="line-number">6</span><br><span class="line-number">7</span><br><span class="line-number">8</span><br></div></div><h2 id="netzwerk-http-request-und-download" tabindex="-1">Netzwerk: HTTP-Request und Download <a class="header-anchor" href="#netzwerk-http-request-und-download" aria-label="Permalink to &quot;Netzwerk: HTTP-Request und Download&quot;">​</a></h2><div class="language-hyp vp-adaptive-theme line-numbers-mode"><button title="Copy Code" class="copy"></button><span class="lang">hyp</span><pre class="shiki shiki-themes github-light github-dark vp-code" tabindex="0"><code><span class="line"><span>Focus {</span></span>
<span class="line"><span>    entrance {</span></span>
<span class="line"><span>        induce url = &quot;https://example.com&quot;;</span></span>
<span class="line"><span>        induce response = HttpGet(url);</span></span>
<span class="line"><span>        observe &quot;HTTP-Response: &quot; + Substring(response, 0, 100) + &quot;...&quot;;</span></span>
<span class="line"><span>        DownloadFile(url + &quot;/file.txt&quot;, &quot;local.txt&quot;);</span></span>
<span class="line"><span>        observe &quot;Datei heruntergeladen als local.txt&quot;;</span></span>
<span class="line"><span>    }</span></span>
<span class="line"><span>} Relax;</span></span></code></pre><div class="line-numbers-wrapper" aria-hidden="true"><span class="line-number">1</span><br><span class="line-number">2</span><br><span class="line-number">3</span><br><span class="line-number">4</span><br><span class="line-number">5</span><br><span class="line-number">6</span><br><span class="line-number">7</span><br><span class="line-number">8</span><br><span class="line-number">9</span><br></div></div><h2 id="fehlerbehandlung-bei-dateioperationen" tabindex="-1">Fehlerbehandlung bei Dateioperationen <a class="header-anchor" href="#fehlerbehandlung-bei-dateioperationen" aria-label="Permalink to &quot;Fehlerbehandlung bei Dateioperationen&quot;">​</a></h2><div class="language-hyp vp-adaptive-theme line-numbers-mode"><button title="Copy Code" class="copy"></button><span class="lang">hyp</span><pre class="shiki shiki-themes github-light github-dark vp-code" tabindex="0"><code><span class="line"><span>Focus {</span></span>
<span class="line"><span>    Trance safeRead(path) {</span></span>
<span class="line"><span>        try {</span></span>
<span class="line"><span>            return ReadFile(path);</span></span>
<span class="line"><span>        } catch (error) {</span></span>
<span class="line"><span>            return &quot;Fehler beim Lesen: &quot; + error;</span></span>
<span class="line"><span>        }</span></span>
<span class="line"><span>    }</span></span>
<span class="line"><span>    entrance {</span></span>
<span class="line"><span>        observe safeRead(&quot;nicht_existierend.txt&quot;);</span></span>
<span class="line"><span>    }</span></span>
<span class="line"><span>} Relax;</span></span></code></pre><div class="line-numbers-wrapper" aria-hidden="true"><span class="line-number">1</span><br><span class="line-number">2</span><br><span class="line-number">3</span><br><span class="line-number">4</span><br><span class="line-number">5</span><br><span class="line-number">6</span><br><span class="line-number">7</span><br><span class="line-number">8</span><br><span class="line-number">9</span><br><span class="line-number">10</span><br><span class="line-number">11</span><br><span class="line-number">12</span><br></div></div><h2 id="kombinierte-system-workflows" tabindex="-1">Kombinierte System-Workflows <a class="header-anchor" href="#kombinierte-system-workflows" aria-label="Permalink to &quot;Kombinierte System-Workflows&quot;">​</a></h2><div class="language-hyp vp-adaptive-theme line-numbers-mode"><button title="Copy Code" class="copy"></button><span class="lang">hyp</span><pre class="shiki shiki-themes github-light github-dark vp-code" tabindex="0"><code><span class="line"><span>Focus {</span></span>
<span class="line"><span>    entrance {</span></span>
<span class="line"><span>        // Backup und Monitoring kombiniert</span></span>
<span class="line"><span>        induce file = &quot;daten.txt&quot;;</span></span>
<span class="line"><span>        if (FileExists(file)) {</span></span>
<span class="line"><span>            induce backup = file + &quot;.bak&quot;;</span></span>
<span class="line"><span>            CopyFile(file, backup);</span></span>
<span class="line"><span>            observe &quot;Backup erstellt: &quot; + backup;</span></span>
<span class="line"><span>        }</span></span>
<span class="line"><span>        induce sys = GetSystemInfo();</span></span>
<span class="line"><span>        observe &quot;System: &quot; + sys.os + &quot; (&quot; + sys.architecture + &quot;)&quot;;</span></span>
<span class="line"><span>    }</span></span>
<span class="line"><span>} Relax;</span></span></code></pre><div class="line-numbers-wrapper" aria-hidden="true"><span class="line-number">1</span><br><span class="line-number">2</span><br><span class="line-number">3</span><br><span class="line-number">4</span><br><span class="line-number">5</span><br><span class="line-number">6</span><br><span class="line-number">7</span><br><span class="line-number">8</span><br><span class="line-number">9</span><br><span class="line-number">10</span><br><span class="line-number">11</span><br><span class="line-number">12</span><br><span class="line-number">13</span><br></div></div><hr><p><strong>Siehe auch:</strong></p><ul><li><a href="./../builtins/system-functions.html">System-Funktionen Referenz</a></li><li><a href="./utility-examples.html">Utility-Funktionen Beispiele</a></li></ul>`,23)])])}const m=s(l,[["render",i]]);export{d as __pageData,m as default};
