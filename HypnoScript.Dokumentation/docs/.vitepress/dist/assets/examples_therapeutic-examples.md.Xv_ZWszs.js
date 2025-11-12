import{_ as s,c as a,o as e,ag as p}from"./chunks/framework.Dli2S8Ej.js";const m=JSON.parse('{"title":"Therapeutic Applications","description":"","frontmatter":{"title":"Therapeutic Applications"},"headers":[],"relativePath":"examples/therapeutic-examples.md","filePath":"examples/therapeutic-examples.md","lastUpdated":1750802968000}'),i={name:"examples/therapeutic-examples.md"};function l(r,n,t,c,o,u){return e(),a("div",null,[...n[0]||(n[0]=[p(`<h1 id="therapeutic-applications" tabindex="-1">Therapeutic Applications <a class="header-anchor" href="#therapeutic-applications" aria-label="Permalink to &quot;Therapeutic Applications&quot;">​</a></h1><p>This page contains therapeutic applications and examples using HypnoScript&#39;s hypnotic functions.</p><h2 id="overview" tabindex="-1">Overview <a class="header-anchor" href="#overview" aria-label="Permalink to &quot;Overview&quot;">​</a></h2><p>HypnoScript provides powerful tools for therapeutic applications including anxiety reduction, pain management, habit change, and more.</p><h2 id="anxiety-reduction" tabindex="-1">Anxiety Reduction <a class="header-anchor" href="#anxiety-reduction" aria-label="Permalink to &quot;Anxiety Reduction&quot;">​</a></h2><h3 id="general-anxiety" tabindex="-1">General Anxiety <a class="header-anchor" href="#general-anxiety" aria-label="Permalink to &quot;General Anxiety&quot;">​</a></h3><div class="language-hyp vp-adaptive-theme line-numbers-mode"><button title="Copy Code" class="copy"></button><span class="lang">hyp</span><pre class="shiki shiki-themes github-light github-dark vp-code" tabindex="0"><code><span class="line"><span>Focus {</span></span>
<span class="line"><span>    entrance {</span></span>
<span class="line"><span>        // Safety check</span></span>
<span class="line"><span>        induce safety = SafetyCheck();</span></span>
<span class="line"><span>        if (!safety.isSafe) {</span></span>
<span class="line"><span>            observe &quot;Session not safe - aborting&quot;;</span></span>
<span class="line"><span>            return;</span></span>
<span class="line"><span>        }</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        // Anxiety reduction session</span></span>
<span class="line"><span>        observe &quot;Welcome to your anxiety reduction session&quot;;</span></span>
<span class="line"><span>        drift(2000);</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        // Progressive relaxation</span></span>
<span class="line"><span>        ProgressiveRelaxation(3);</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        // Anxiety-specific breathing</span></span>
<span class="line"><span>        HypnoticBreathing(7);</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        // Anxiety reduction</span></span>
<span class="line"><span>        AnxietyReduction(&quot;general&quot;, 0.8);</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        // Positive suggestions</span></span>
<span class="line"><span>        HypnoticSuggestion(&quot;You feel increasingly calm and secure&quot;, 3);</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        // Grounding</span></span>
<span class="line"><span>        Grounding(&quot;visual&quot;, 60);</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        observe &quot;Anxiety reduction session completed&quot;;</span></span>
<span class="line"><span>    }</span></span>
<span class="line"><span>} Relax;</span></span></code></pre><div class="line-numbers-wrapper" aria-hidden="true"><span class="line-number">1</span><br><span class="line-number">2</span><br><span class="line-number">3</span><br><span class="line-number">4</span><br><span class="line-number">5</span><br><span class="line-number">6</span><br><span class="line-number">7</span><br><span class="line-number">8</span><br><span class="line-number">9</span><br><span class="line-number">10</span><br><span class="line-number">11</span><br><span class="line-number">12</span><br><span class="line-number">13</span><br><span class="line-number">14</span><br><span class="line-number">15</span><br><span class="line-number">16</span><br><span class="line-number">17</span><br><span class="line-number">18</span><br><span class="line-number">19</span><br><span class="line-number">20</span><br><span class="line-number">21</span><br><span class="line-number">22</span><br><span class="line-number">23</span><br><span class="line-number">24</span><br><span class="line-number">25</span><br><span class="line-number">26</span><br><span class="line-number">27</span><br><span class="line-number">28</span><br><span class="line-number">29</span><br><span class="line-number">30</span><br><span class="line-number">31</span><br></div></div><h3 id="specific-phobias" tabindex="-1">Specific Phobias <a class="header-anchor" href="#specific-phobias" aria-label="Permalink to &quot;Specific Phobias&quot;">​</a></h3><div class="language-hyp vp-adaptive-theme line-numbers-mode"><button title="Copy Code" class="copy"></button><span class="lang">hyp</span><pre class="shiki shiki-themes github-light github-dark vp-code" tabindex="0"><code><span class="line"><span>Focus {</span></span>
<span class="line"><span>    entrance {</span></span>
<span class="line"><span>        induce phobia = InputProvider(&quot;What is your specific fear? &quot;);</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        // Phobia-specific work</span></span>
<span class="line"><span>        if (phobia == &quot;spiders&quot;) {</span></span>
<span class="line"><span>            HypnoticVisualization(&quot;a gentle, harmless spider&quot;, 30);</span></span>
<span class="line"><span>            HypnoticSuggestion(&quot;You feel calm and in control around spiders&quot;, 3);</span></span>
<span class="line"><span>        } else if (phobia == &quot;heights&quot;) {</span></span>
<span class="line"><span>            HypnoticVisualization(&quot;standing safely on a mountain top&quot;, 30);</span></span>
<span class="line"><span>            HypnoticSuggestion(&quot;You feel secure and balanced at any height&quot;, 3);</span></span>
<span class="line"><span>        }</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        // Desensitization</span></span>
<span class="line"><span>        observe &quot;Phobia desensitization completed&quot;;</span></span>
<span class="line"><span>    }</span></span>
<span class="line"><span>} Relax;</span></span></code></pre><div class="line-numbers-wrapper" aria-hidden="true"><span class="line-number">1</span><br><span class="line-number">2</span><br><span class="line-number">3</span><br><span class="line-number">4</span><br><span class="line-number">5</span><br><span class="line-number">6</span><br><span class="line-number">7</span><br><span class="line-number">8</span><br><span class="line-number">9</span><br><span class="line-number">10</span><br><span class="line-number">11</span><br><span class="line-number">12</span><br><span class="line-number">13</span><br><span class="line-number">14</span><br><span class="line-number">15</span><br><span class="line-number">16</span><br><span class="line-number">17</span><br></div></div><h2 id="pain-management" tabindex="-1">Pain Management <a class="header-anchor" href="#pain-management" aria-label="Permalink to &quot;Pain Management&quot;">​</a></h2><h3 id="chronic-pain" tabindex="-1">Chronic Pain <a class="header-anchor" href="#chronic-pain" aria-label="Permalink to &quot;Chronic Pain&quot;">​</a></h3><div class="language-hyp vp-adaptive-theme line-numbers-mode"><button title="Copy Code" class="copy"></button><span class="lang">hyp</span><pre class="shiki shiki-themes github-light github-dark vp-code" tabindex="0"><code><span class="line"><span>Focus {</span></span>
<span class="line"><span>    entrance {</span></span>
<span class="line"><span>        induce painType = InputProvider(&quot;Type of pain: &quot;);</span></span>
<span class="line"><span>        induce painLevel = InputProvider(&quot;Pain level (1-10): &quot;);</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        // Pain management session</span></span>
<span class="line"><span>        PainManagement(&quot;reduce&quot;, painType);</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        // Pain visualization</span></span>
<span class="line"><span>        HypnoticVisualization(&quot;pain as a color that fades away&quot;, 45);</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        // Pain control suggestions</span></span>
<span class="line"><span>        HypnoticSuggestion(&quot;You have control over your pain&quot;, 3);</span></span>
<span class="line"><span>        HypnoticSuggestion(&quot;Your pain is decreasing with each breath&quot;, 3);</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        observe &quot;Pain management session completed&quot;;</span></span>
<span class="line"><span>    }</span></span>
<span class="line"><span>} Relax;</span></span></code></pre><div class="line-numbers-wrapper" aria-hidden="true"><span class="line-number">1</span><br><span class="line-number">2</span><br><span class="line-number">3</span><br><span class="line-number">4</span><br><span class="line-number">5</span><br><span class="line-number">6</span><br><span class="line-number">7</span><br><span class="line-number">8</span><br><span class="line-number">9</span><br><span class="line-number">10</span><br><span class="line-number">11</span><br><span class="line-number">12</span><br><span class="line-number">13</span><br><span class="line-number">14</span><br><span class="line-number">15</span><br><span class="line-number">16</span><br><span class="line-number">17</span><br><span class="line-number">18</span><br></div></div><h3 id="acute-pain" tabindex="-1">Acute Pain <a class="header-anchor" href="#acute-pain" aria-label="Permalink to &quot;Acute Pain&quot;">​</a></h3><div class="language-hyp vp-adaptive-theme line-numbers-mode"><button title="Copy Code" class="copy"></button><span class="lang">hyp</span><pre class="shiki shiki-themes github-light github-dark vp-code" tabindex="0"><code><span class="line"><span>Focus {</span></span>
<span class="line"><span>    entrance {</span></span>
<span class="line"><span>        // Quick pain relief</span></span>
<span class="line"><span>        HypnoticBreathing(5);</span></span>
<span class="line"><span>        PainManagement(&quot;relieve&quot;, &quot;acute&quot;);</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        // Emergency pain control</span></span>
<span class="line"><span>        HypnoticSuggestion(&quot;Your pain is being managed effectively&quot;, 2);</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        observe &quot;Acute pain relief applied&quot;;</span></span>
<span class="line"><span>    }</span></span>
<span class="line"><span>} Relax;</span></span></code></pre><div class="line-numbers-wrapper" aria-hidden="true"><span class="line-number">1</span><br><span class="line-number">2</span><br><span class="line-number">3</span><br><span class="line-number">4</span><br><span class="line-number">5</span><br><span class="line-number">6</span><br><span class="line-number">7</span><br><span class="line-number">8</span><br><span class="line-number">9</span><br><span class="line-number">10</span><br><span class="line-number">11</span><br><span class="line-number">12</span><br></div></div><h2 id="habit-change" tabindex="-1">Habit Change <a class="header-anchor" href="#habit-change" aria-label="Permalink to &quot;Habit Change&quot;">​</a></h2><h3 id="smoking-cessation" tabindex="-1">Smoking Cessation <a class="header-anchor" href="#smoking-cessation" aria-label="Permalink to &quot;Smoking Cessation&quot;">​</a></h3><div class="language-hyp vp-adaptive-theme line-numbers-mode"><button title="Copy Code" class="copy"></button><span class="lang">hyp</span><pre class="shiki shiki-themes github-light github-dark vp-code" tabindex="0"><code><span class="line"><span>Focus {</span></span>
<span class="line"><span>    entrance {</span></span>
<span class="line"><span>        // Identify smoking habit</span></span>
<span class="line"><span>        induce habit = HabitChange(&quot;identify&quot;, &quot;smoking&quot;);</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        // Replace with healthy alternative</span></span>
<span class="line"><span>        HabitChange(&quot;modify&quot;, habit, &quot;deep breathing&quot;);</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        // Reinforcement</span></span>
<span class="line"><span>        HypnoticSuggestion(&quot;You prefer healthy breathing over smoking&quot;, 3);</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        observe &quot;Smoking cessation session completed&quot;;</span></span>
<span class="line"><span>    }</span></span>
<span class="line"><span>} Relax;</span></span></code></pre><div class="line-numbers-wrapper" aria-hidden="true"><span class="line-number">1</span><br><span class="line-number">2</span><br><span class="line-number">3</span><br><span class="line-number">4</span><br><span class="line-number">5</span><br><span class="line-number">6</span><br><span class="line-number">7</span><br><span class="line-number">8</span><br><span class="line-number">9</span><br><span class="line-number">10</span><br><span class="line-number">11</span><br><span class="line-number">12</span><br><span class="line-number">13</span><br><span class="line-number">14</span><br></div></div><h3 id="weight-management" tabindex="-1">Weight Management <a class="header-anchor" href="#weight-management" aria-label="Permalink to &quot;Weight Management&quot;">​</a></h3><div class="language-hyp vp-adaptive-theme line-numbers-mode"><button title="Copy Code" class="copy"></button><span class="lang">hyp</span><pre class="shiki shiki-themes github-light github-dark vp-code" tabindex="0"><code><span class="line"><span>Focus {</span></span>
<span class="line"><span>    entrance {</span></span>
<span class="line"><span>        // Identify eating patterns</span></span>
<span class="line"><span>        induce eatingHabit = HabitChange(&quot;identify&quot;, &quot;emotional eating&quot;);</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        // Modify behavior</span></span>
<span class="line"><span>        HabitChange(&quot;modify&quot;, eatingHabit, &quot;mindful eating&quot;);</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        // Positive body image</span></span>
<span class="line"><span>        HypnoticSuggestion(&quot;You have a healthy relationship with food&quot;, 3);</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        observe &quot;Weight management session completed&quot;;</span></span>
<span class="line"><span>    }</span></span>
<span class="line"><span>} Relax;</span></span></code></pre><div class="line-numbers-wrapper" aria-hidden="true"><span class="line-number">1</span><br><span class="line-number">2</span><br><span class="line-number">3</span><br><span class="line-number">4</span><br><span class="line-number">5</span><br><span class="line-number">6</span><br><span class="line-number">7</span><br><span class="line-number">8</span><br><span class="line-number">9</span><br><span class="line-number">10</span><br><span class="line-number">11</span><br><span class="line-number">12</span><br><span class="line-number">13</span><br><span class="line-number">14</span><br></div></div><h2 id="trauma-processing" tabindex="-1">Trauma Processing <a class="header-anchor" href="#trauma-processing" aria-label="Permalink to &quot;Trauma Processing&quot;">​</a></h2><h3 id="ptsd-treatment" tabindex="-1">PTSD Treatment <a class="header-anchor" href="#ptsd-treatment" aria-label="Permalink to &quot;PTSD Treatment&quot;">​</a></h3><div class="language-hyp vp-adaptive-theme line-numbers-mode"><button title="Copy Code" class="copy"></button><span class="lang">hyp</span><pre class="shiki shiki-themes github-light github-dark vp-code" tabindex="0"><code><span class="line"><span>Focus {</span></span>
<span class="line"><span>    entrance {</span></span>
<span class="line"><span>        // Safety first</span></span>
<span class="line"><span>        if (!SafetyCheck().isSafe) {</span></span>
<span class="line"><span>            observe &quot;Client not ready for trauma work&quot;;</span></span>
<span class="line"><span>            return;</span></span>
<span class="line"><span>        }</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        // Safe place creation</span></span>
<span class="line"><span>        HypnoticVisualization(&quot;your safe, peaceful place&quot;, 60);</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        // Trauma processing (supervised)</span></span>
<span class="line"><span>        observe &quot;Trauma processing session - professional supervision required&quot;;</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        // Grounding</span></span>
<span class="line"><span>        Grounding(&quot;physical&quot;, 90);</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        observe &quot;Trauma processing session completed&quot;;</span></span>
<span class="line"><span>    }</span></span>
<span class="line"><span>} Relax;</span></span></code></pre><div class="line-numbers-wrapper" aria-hidden="true"><span class="line-number">1</span><br><span class="line-number">2</span><br><span class="line-number">3</span><br><span class="line-number">4</span><br><span class="line-number">5</span><br><span class="line-number">6</span><br><span class="line-number">7</span><br><span class="line-number">8</span><br><span class="line-number">9</span><br><span class="line-number">10</span><br><span class="line-number">11</span><br><span class="line-number">12</span><br><span class="line-number">13</span><br><span class="line-number">14</span><br><span class="line-number">15</span><br><span class="line-number">16</span><br><span class="line-number">17</span><br><span class="line-number">18</span><br><span class="line-number">19</span><br><span class="line-number">20</span><br></div></div><h2 id="depression-support" tabindex="-1">Depression Support <a class="header-anchor" href="#depression-support" aria-label="Permalink to &quot;Depression Support&quot;">​</a></h2><h3 id="mood-elevation" tabindex="-1">Mood Elevation <a class="header-anchor" href="#mood-elevation" aria-label="Permalink to &quot;Mood Elevation&quot;">​</a></h3><div class="language-hyp vp-adaptive-theme line-numbers-mode"><button title="Copy Code" class="copy"></button><span class="lang">hyp</span><pre class="shiki shiki-themes github-light github-dark vp-code" tabindex="0"><code><span class="line"><span>Focus {</span></span>
<span class="line"><span>    entrance {</span></span>
<span class="line"><span>        // Depression assessment</span></span>
<span class="line"><span>        induce moodLevel = InputProvider(&quot;Current mood level (1-10): &quot;);</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        if (moodLevel &lt; 4) {</span></span>
<span class="line"><span>            observe &quot;Severe depression - professional help recommended&quot;;</span></span>
<span class="line"><span>            return;</span></span>
<span class="line"><span>        }</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        // Mood elevation techniques</span></span>
<span class="line"><span>        HypnoticVisualization(&quot;a bright, sunny day&quot;, 45);</span></span>
<span class="line"><span>        HypnoticSuggestion(&quot;You feel increasingly positive and hopeful&quot;, 3);</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        // Future progression</span></span>
<span class="line"><span>        HypnoticFutureProgression(1); // 1 year ahead</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        observe &quot;Mood elevation session completed&quot;;</span></span>
<span class="line"><span>    }</span></span>
<span class="line"><span>} Relax;</span></span></code></pre><div class="line-numbers-wrapper" aria-hidden="true"><span class="line-number">1</span><br><span class="line-number">2</span><br><span class="line-number">3</span><br><span class="line-number">4</span><br><span class="line-number">5</span><br><span class="line-number">6</span><br><span class="line-number">7</span><br><span class="line-number">8</span><br><span class="line-number">9</span><br><span class="line-number">10</span><br><span class="line-number">11</span><br><span class="line-number">12</span><br><span class="line-number">13</span><br><span class="line-number">14</span><br><span class="line-number">15</span><br><span class="line-number">16</span><br><span class="line-number">17</span><br><span class="line-number">18</span><br><span class="line-number">19</span><br><span class="line-number">20</span><br></div></div><h2 id="sleep-improvement" tabindex="-1">Sleep Improvement <a class="header-anchor" href="#sleep-improvement" aria-label="Permalink to &quot;Sleep Improvement&quot;">​</a></h2><h3 id="insomnia-treatment" tabindex="-1">Insomnia Treatment <a class="header-anchor" href="#insomnia-treatment" aria-label="Permalink to &quot;Insomnia Treatment&quot;">​</a></h3><div class="language-hyp vp-adaptive-theme line-numbers-mode"><button title="Copy Code" class="copy"></button><span class="lang">hyp</span><pre class="shiki shiki-themes github-light github-dark vp-code" tabindex="0"><code><span class="line"><span>Focus {</span></span>
<span class="line"><span>    entrance {</span></span>
<span class="line"><span>        // Sleep preparation</span></span>
<span class="line"><span>        ProgressiveRelaxation(2);</span></span>
<span class="line"><span>        HypnoticBreathing(10);</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        // Sleep suggestions</span></span>
<span class="line"><span>        HypnoticSuggestion(&quot;You will sleep deeply and peacefully&quot;, 3);</span></span>
<span class="line"><span>        HypnoticSuggestion(&quot;You wake up refreshed and energized&quot;, 2);</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        // Sleep visualization</span></span>
<span class="line"><span>        HypnoticVisualization(&quot;floating on a cloud of sleep&quot;, 60);</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        observe &quot;Sleep improvement session completed&quot;;</span></span>
<span class="line"><span>    }</span></span>
<span class="line"><span>} Relax;</span></span></code></pre><div class="line-numbers-wrapper" aria-hidden="true"><span class="line-number">1</span><br><span class="line-number">2</span><br><span class="line-number">3</span><br><span class="line-number">4</span><br><span class="line-number">5</span><br><span class="line-number">6</span><br><span class="line-number">7</span><br><span class="line-number">8</span><br><span class="line-number">9</span><br><span class="line-number">10</span><br><span class="line-number">11</span><br><span class="line-number">12</span><br><span class="line-number">13</span><br><span class="line-number">14</span><br><span class="line-number">15</span><br><span class="line-number">16</span><br></div></div><h2 id="best-practices" tabindex="-1">Best Practices <a class="header-anchor" href="#best-practices" aria-label="Permalink to &quot;Best Practices&quot;">​</a></h2><h3 id="session-structure" tabindex="-1">Session Structure <a class="header-anchor" href="#session-structure" aria-label="Permalink to &quot;Session Structure&quot;">​</a></h3><ol><li><strong>Safety Check</strong> - Always begin with SafetyCheck()</li><li><strong>Assessment</strong> - Understand the client&#39;s specific needs</li><li><strong>Induction</strong> - Gentle trance induction</li><li><strong>Therapeutic Work</strong> - Specific interventions</li><li><strong>Integration</strong> - Help client integrate changes</li><li><strong>Grounding</strong> - Proper session closure</li></ol><h3 id="professional-guidelines" tabindex="-1">Professional Guidelines <a class="header-anchor" href="#professional-guidelines" aria-label="Permalink to &quot;Professional Guidelines&quot;">​</a></h3><ul><li>Always work within your scope of practice</li><li>Refer to mental health professionals when appropriate</li><li>Maintain proper documentation</li><li>Follow ethical guidelines</li><li>Ensure informed consent</li></ul><h3 id="monitoring-progress" tabindex="-1">Monitoring Progress <a class="header-anchor" href="#monitoring-progress" aria-label="Permalink to &quot;Monitoring Progress&quot;">​</a></h3><div class="language-hyp vp-adaptive-theme line-numbers-mode"><button title="Copy Code" class="copy"></button><span class="lang">hyp</span><pre class="shiki shiki-themes github-light github-dark vp-code" tabindex="0"><code><span class="line"><span>Focus {</span></span>
<span class="line"><span>    entrance {</span></span>
<span class="line"><span>        // Progress tracking</span></span>
<span class="line"><span>        induce sessionNumber = InputProvider(&quot;Session number: &quot;);</span></span>
<span class="line"><span>        induce progress = InputProvider(&quot;Progress rating (1-10): &quot;);</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        // Record progress</span></span>
<span class="line"><span>        observe &quot;Session &quot; + sessionNumber + &quot; completed&quot;;</span></span>
<span class="line"><span>        observe &quot;Progress rating: &quot; + progress + &quot;/10&quot;;</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        // Adjust treatment plan</span></span>
<span class="line"><span>        if (progress &lt; 5) {</span></span>
<span class="line"><span>            observe &quot;Consider adjusting treatment approach&quot;;</span></span>
<span class="line"><span>        }</span></span>
<span class="line"><span>    }</span></span>
<span class="line"><span>} Relax;</span></span></code></pre><div class="line-numbers-wrapper" aria-hidden="true"><span class="line-number">1</span><br><span class="line-number">2</span><br><span class="line-number">3</span><br><span class="line-number">4</span><br><span class="line-number">5</span><br><span class="line-number">6</span><br><span class="line-number">7</span><br><span class="line-number">8</span><br><span class="line-number">9</span><br><span class="line-number">10</span><br><span class="line-number">11</span><br><span class="line-number">12</span><br><span class="line-number">13</span><br><span class="line-number">14</span><br><span class="line-number">15</span><br><span class="line-number">16</span><br></div></div><h2 id="emergency-procedures" tabindex="-1">Emergency Procedures <a class="header-anchor" href="#emergency-procedures" aria-label="Permalink to &quot;Emergency Procedures&quot;">​</a></h2><h3 id="crisis-intervention" tabindex="-1">Crisis Intervention <a class="header-anchor" href="#crisis-intervention" aria-label="Permalink to &quot;Crisis Intervention&quot;">​</a></h3><div class="language-hyp vp-adaptive-theme line-numbers-mode"><button title="Copy Code" class="copy"></button><span class="lang">hyp</span><pre class="shiki shiki-themes github-light github-dark vp-code" tabindex="0"><code><span class="line"><span>Focus {</span></span>
<span class="line"><span>    entrance {</span></span>
<span class="line"><span>        // Emergency assessment</span></span>
<span class="line"><span>        induce crisisLevel = InputProvider(&quot;Crisis level (1-10): &quot;);</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        if (crisisLevel &gt; 7) {</span></span>
<span class="line"><span>            observe &quot;CRISIS: Immediate professional intervention required&quot;;</span></span>
<span class="line"><span>            EmergencyExit(&quot;immediate&quot;);</span></span>
<span class="line"><span>            return;</span></span>
<span class="line"><span>        }</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        // Crisis stabilization</span></span>
<span class="line"><span>        HypnoticBreathing(5);</span></span>
<span class="line"><span>        Grounding(&quot;physical&quot;, 120);</span></span>
<span class="line"><span></span></span>
<span class="line"><span>        observe &quot;Crisis stabilized - follow-up care needed&quot;;</span></span>
<span class="line"><span>    }</span></span>
<span class="line"><span>} Relax;</span></span></code></pre><div class="line-numbers-wrapper" aria-hidden="true"><span class="line-number">1</span><br><span class="line-number">2</span><br><span class="line-number">3</span><br><span class="line-number">4</span><br><span class="line-number">5</span><br><span class="line-number">6</span><br><span class="line-number">7</span><br><span class="line-number">8</span><br><span class="line-number">9</span><br><span class="line-number">10</span><br><span class="line-number">11</span><br><span class="line-number">12</span><br><span class="line-number">13</span><br><span class="line-number">14</span><br><span class="line-number">15</span><br><span class="line-number">16</span><br><span class="line-number">17</span><br><span class="line-number">18</span><br></div></div><h2 id="integration-with-other-therapies" tabindex="-1">Integration with Other Therapies <a class="header-anchor" href="#integration-with-other-therapies" aria-label="Permalink to &quot;Integration with Other Therapies&quot;">​</a></h2><p>HypnoScript can be effectively integrated with:</p><ul><li>Cognitive Behavioral Therapy (CBT)</li><li>Mindfulness practices</li><li>Traditional psychotherapy</li><li>Medical treatments</li><li>Physical therapy</li></ul><h2 id="next-steps" tabindex="-1">Next Steps <a class="header-anchor" href="#next-steps" aria-label="Permalink to &quot;Next Steps&quot;">​</a></h2><ul><li><a href="./basic-examples.html">Basic Examples</a> - Basic usage examples</li><li><a href="./system-examples.html">System Examples</a> - System integration examples</li><li><a href="./cli-workflows.html">CLI Workflows</a> - Command-line workflows</li></ul><hr><p><strong>Ready to explore more therapeutic applications? Check out the <a href="./basic-examples.html">Basic Examples</a>!</strong> ✅</p>`,45)])])}const d=s(i,[["render",l]]);export{m as __pageData,d as default};
