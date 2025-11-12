import{_ as n,c as a,o as e,ag as p}from"./chunks/framework.Dli2S8Ej.js";const m=JSON.parse('{"title":"Testing Fixtures","description":"","frontmatter":{"title":"Testing Fixtures"},"headers":[],"relativePath":"testing/fixtures.md","filePath":"testing/fixtures.md","lastUpdated":1750802436000}'),l={name:"testing/fixtures.md"};function i(r,s,t,u,c,o){return e(),a("div",null,[...s[0]||(s[0]=[p(`<h1 id="test-fixtures" tabindex="-1">Test Fixtures <a class="header-anchor" href="#test-fixtures" aria-label="Permalink to &quot;Test Fixtures&quot;">​</a></h1><p>Test fixtures provide a way to set up test data and environments for consistent, repeatable testing in HypnoScript.</p><h2 id="overview" tabindex="-1">Overview <a class="header-anchor" href="#overview" aria-label="Permalink to &quot;Overview&quot;">​</a></h2><p>Test fixtures are predefined data sets and configurations that help ensure your tests run consistently across different environments and scenarios.</p><h2 id="creating-test-fixtures" tabindex="-1">Creating Test Fixtures <a class="header-anchor" href="#creating-test-fixtures" aria-label="Permalink to &quot;Creating Test Fixtures&quot;">​</a></h2><h3 id="_1-basic-test-fixture-structure" tabindex="-1">1. Basic Test Fixture Structure <a class="header-anchor" href="#_1-basic-test-fixture-structure" aria-label="Permalink to &quot;1. Basic Test Fixture Structure&quot;">​</a></h3><div class="language-hyp vp-adaptive-theme line-numbers-mode"><button title="Copy Code" class="copy"></button><span class="lang">hyp</span><pre class="shiki shiki-themes github-light github-dark vp-code" tabindex="0"><code><span class="line"><span>// test_fixtures.hyp</span></span>
<span class="line"><span>Session TestData {</span></span>
<span class="line"><span>  // User data fixtures</span></span>
<span class="line"><span>  induce testUser: record = {</span></span>
<span class="line"><span>    &quot;name&quot;: &quot;John Doe&quot;,</span></span>
<span class="line"><span>    &quot;email&quot;: &quot;john@example.com&quot;,</span></span>
<span class="line"><span>    &quot;age&quot;: 30,</span></span>
<span class="line"><span>    &quot;active&quot;: true</span></span>
<span class="line"><span>  };</span></span>
<span class="line"><span></span></span>
<span class="line"><span>  induce adminUser: record = {</span></span>
<span class="line"><span>    &quot;name&quot;: &quot;Admin User&quot;,</span></span>
<span class="line"><span>    &quot;email&quot;: &quot;admin@example.com&quot;,</span></span>
<span class="line"><span>    &quot;age&quot;: 35,</span></span>
<span class="line"><span>    &quot;active&quot;: true,</span></span>
<span class="line"><span>    &quot;role&quot;: &quot;admin&quot;</span></span>
<span class="line"><span>  };</span></span>
<span class="line"><span></span></span>
<span class="line"><span>  // Array fixtures</span></span>
<span class="line"><span>  induce numberArray: number[] = [1, 2, 3, 4, 5, 10, 15, 20];</span></span>
<span class="line"><span>  induce stringArray: string[] = [&quot;apple&quot;, &quot;banana&quot;, &quot;cherry&quot;, &quot;date&quot;];</span></span>
<span class="line"><span>  induce mixedArray: any[] = [1, &quot;hello&quot;, true, 3.14];</span></span>
<span class="line"><span></span></span>
<span class="line"><span>  // Configuration fixtures</span></span>
<span class="line"><span>  induce testConfig: record = {</span></span>
<span class="line"><span>    &quot;timeout&quot;: 5000,</span></span>
<span class="line"><span>    &quot;retries&quot;: 3,</span></span>
<span class="line"><span>    &quot;debug&quot;: true,</span></span>
<span class="line"><span>    &quot;logLevel&quot;: &quot;INFO&quot;</span></span>
<span class="line"><span>  };</span></span>
<span class="line"><span>}</span></span></code></pre><div class="line-numbers-wrapper" aria-hidden="true"><span class="line-number">1</span><br><span class="line-number">2</span><br><span class="line-number">3</span><br><span class="line-number">4</span><br><span class="line-number">5</span><br><span class="line-number">6</span><br><span class="line-number">7</span><br><span class="line-number">8</span><br><span class="line-number">9</span><br><span class="line-number">10</span><br><span class="line-number">11</span><br><span class="line-number">12</span><br><span class="line-number">13</span><br><span class="line-number">14</span><br><span class="line-number">15</span><br><span class="line-number">16</span><br><span class="line-number">17</span><br><span class="line-number">18</span><br><span class="line-number">19</span><br><span class="line-number">20</span><br><span class="line-number">21</span><br><span class="line-number">22</span><br><span class="line-number">23</span><br><span class="line-number">24</span><br><span class="line-number">25</span><br><span class="line-number">26</span><br><span class="line-number">27</span><br><span class="line-number">28</span><br><span class="line-number">29</span><br><span class="line-number">30</span><br><span class="line-number">31</span><br></div></div><h3 id="_2-loading-fixtures-in-tests" tabindex="-1">2. Loading Fixtures in Tests <a class="header-anchor" href="#_2-loading-fixtures-in-tests" aria-label="Permalink to &quot;2. Loading Fixtures in Tests&quot;">​</a></h3><div class="language-hyp vp-adaptive-theme line-numbers-mode"><button title="Copy Code" class="copy"></button><span class="lang">hyp</span><pre class="shiki shiki-themes github-light github-dark vp-code" tabindex="0"><code><span class="line"><span>// test_with_fixtures.hyp</span></span>
<span class="line"><span>Focus {</span></span>
<span class="line"><span>  // Load test fixtures</span></span>
<span class="line"><span>  MindLink TestData;</span></span>
<span class="line"><span></span></span>
<span class="line"><span>  // Use fixture data in tests</span></span>
<span class="line"><span>  induce user: record = testUser;</span></span>
<span class="line"><span>  Observe(&quot;Testing with user: &quot; + user[&quot;name&quot;]);</span></span>
<span class="line"><span></span></span>
<span class="line"><span>  // Validate user data</span></span>
<span class="line"><span>  Assert(IsString(user[&quot;name&quot;]), &quot;User name should be a string&quot;);</span></span>
<span class="line"><span>  Assert(IsNumber(user[&quot;age&quot;]), &quot;User age should be a number&quot;);</span></span>
<span class="line"><span>  Assert(user[&quot;age&quot;] &gt; 0, &quot;User age should be positive&quot;);</span></span>
<span class="line"><span></span></span>
<span class="line"><span>  // Test with different fixtures</span></span>
<span class="line"><span>  induce admin: record = adminUser;</span></span>
<span class="line"><span>  Assert(admin[&quot;role&quot;] == &quot;admin&quot;, &quot;Admin should have admin role&quot;);</span></span>
<span class="line"><span></span></span>
<span class="line"><span>  // Test array fixtures</span></span>
<span class="line"><span>  induce numbers: number[] = numberArray;</span></span>
<span class="line"><span>  Assert(ArrayLength(numbers) == 8, &quot;Number array should have 8 elements&quot;);</span></span>
<span class="line"><span>  Assert(numbers[0] == 1, &quot;First element should be 1&quot;);</span></span>
<span class="line"><span></span></span>
<span class="line"><span>  Observe(&quot;All fixture tests passed!&quot;);</span></span>
<span class="line"><span>} Relax</span></span></code></pre><div class="line-numbers-wrapper" aria-hidden="true"><span class="line-number">1</span><br><span class="line-number">2</span><br><span class="line-number">3</span><br><span class="line-number">4</span><br><span class="line-number">5</span><br><span class="line-number">6</span><br><span class="line-number">7</span><br><span class="line-number">8</span><br><span class="line-number">9</span><br><span class="line-number">10</span><br><span class="line-number">11</span><br><span class="line-number">12</span><br><span class="line-number">13</span><br><span class="line-number">14</span><br><span class="line-number">15</span><br><span class="line-number">16</span><br><span class="line-number">17</span><br><span class="line-number">18</span><br><span class="line-number">19</span><br><span class="line-number">20</span><br><span class="line-number">21</span><br><span class="line-number">22</span><br><span class="line-number">23</span><br><span class="line-number">24</span><br><span class="line-number">25</span><br></div></div><h2 id="advanced-fixture-patterns" tabindex="-1">Advanced Fixture Patterns <a class="header-anchor" href="#advanced-fixture-patterns" aria-label="Permalink to &quot;Advanced Fixture Patterns&quot;">​</a></h2><h3 id="_1-dynamic-fixture-generation" tabindex="-1">1. Dynamic Fixture Generation <a class="header-anchor" href="#_1-dynamic-fixture-generation" aria-label="Permalink to &quot;1. Dynamic Fixture Generation&quot;">​</a></h3><div class="language-hyp vp-adaptive-theme line-numbers-mode"><button title="Copy Code" class="copy"></button><span class="lang">hyp</span><pre class="shiki shiki-themes github-light github-dark vp-code" tabindex="0"><code><span class="line"><span>// dynamic_fixtures.hyp</span></span>
<span class="line"><span>Focus {</span></span>
<span class="line"><span>  function GenerateUserFixture(name: string, age: number, role: string): record {</span></span>
<span class="line"><span>    return {</span></span>
<span class="line"><span>      &quot;name&quot;: name,</span></span>
<span class="line"><span>      &quot;email&quot;: ToLowerCase(name) + &quot;@example.com&quot;,</span></span>
<span class="line"><span>      &quot;age&quot;: age,</span></span>
<span class="line"><span>      &quot;role&quot;: role,</span></span>
<span class="line"><span>      &quot;active&quot;: true,</span></span>
<span class="line"><span>      &quot;createdAt&quot;: GetCurrentTime()</span></span>
<span class="line"><span>    };</span></span>
<span class="line"><span>  }</span></span>
<span class="line"><span></span></span>
<span class="line"><span>  function GenerateNumberArray(size: number, start: number, step: number): number[] {</span></span>
<span class="line"><span>    induce result: number[] = [];</span></span>
<span class="line"><span>    induce current: number = start;</span></span>
<span class="line"><span></span></span>
<span class="line"><span>    for (induce i: number = 0; i &lt; size; i = i + 1) {</span></span>
<span class="line"><span>      result = ArrayPush(result, current);</span></span>
<span class="line"><span>      current = current + step;</span></span>
<span class="line"><span>    }</span></span>
<span class="line"><span></span></span>
<span class="line"><span>    return result;</span></span>
<span class="line"><span>  }</span></span>
<span class="line"><span></span></span>
<span class="line"><span>  // Generate test data dynamically</span></span>
<span class="line"><span>  induce dynamicUser: record = GenerateUserFixture(&quot;Jane Smith&quot;, 28, &quot;user&quot;);</span></span>
<span class="line"><span>  induce fibonacci: number[] = GenerateNumberArray(10, 1, 1);</span></span>
<span class="line"><span></span></span>
<span class="line"><span>  // Test dynamic fixtures</span></span>
<span class="line"><span>  Assert(dynamicUser[&quot;name&quot;] == &quot;Jane Smith&quot;, &quot;Dynamic user name should match&quot;);</span></span>
<span class="line"><span>  Assert(ArrayLength(fibonacci) == 10, &quot;Fibonacci array should have 10 elements&quot;);</span></span>
<span class="line"><span></span></span>
<span class="line"><span>  Observe(&quot;Dynamic fixture generation successful!&quot;);</span></span>
<span class="line"><span>} Relax</span></span></code></pre><div class="line-numbers-wrapper" aria-hidden="true"><span class="line-number">1</span><br><span class="line-number">2</span><br><span class="line-number">3</span><br><span class="line-number">4</span><br><span class="line-number">5</span><br><span class="line-number">6</span><br><span class="line-number">7</span><br><span class="line-number">8</span><br><span class="line-number">9</span><br><span class="line-number">10</span><br><span class="line-number">11</span><br><span class="line-number">12</span><br><span class="line-number">13</span><br><span class="line-number">14</span><br><span class="line-number">15</span><br><span class="line-number">16</span><br><span class="line-number">17</span><br><span class="line-number">18</span><br><span class="line-number">19</span><br><span class="line-number">20</span><br><span class="line-number">21</span><br><span class="line-number">22</span><br><span class="line-number">23</span><br><span class="line-number">24</span><br><span class="line-number">25</span><br><span class="line-number">26</span><br><span class="line-number">27</span><br><span class="line-number">28</span><br><span class="line-number">29</span><br><span class="line-number">30</span><br><span class="line-number">31</span><br><span class="line-number">32</span><br><span class="line-number">33</span><br><span class="line-number">34</span><br><span class="line-number">35</span><br></div></div><h3 id="_2-fixture-validation" tabindex="-1">2. Fixture Validation <a class="header-anchor" href="#_2-fixture-validation" aria-label="Permalink to &quot;2. Fixture Validation&quot;">​</a></h3><div class="language-hyp vp-adaptive-theme line-numbers-mode"><button title="Copy Code" class="copy"></button><span class="lang">hyp</span><pre class="shiki shiki-themes github-light github-dark vp-code" tabindex="0"><code><span class="line"><span>// fixture_validation.hyp</span></span>
<span class="line"><span>Focus {</span></span>
<span class="line"><span>  function ValidateUserFixture(user: record): boolean {</span></span>
<span class="line"><span>    // Check required fields</span></span>
<span class="line"><span>    if (!HasKey(user, &quot;name&quot;) || IsNullOrEmpty(user[&quot;name&quot;])) {</span></span>
<span class="line"><span>      return false;</span></span>
<span class="line"><span>    }</span></span>
<span class="line"><span></span></span>
<span class="line"><span>    if (!HasKey(user, &quot;email&quot;) || IsNullOrEmpty(user[&quot;email&quot;])) {</span></span>
<span class="line"><span>      return false;</span></span>
<span class="line"><span>    }</span></span>
<span class="line"><span></span></span>
<span class="line"><span>    if (!HasKey(user, &quot;age&quot;) || !IsNumber(user[&quot;age&quot;])) {</span></span>
<span class="line"><span>      return false;</span></span>
<span class="line"><span>    }</span></span>
<span class="line"><span></span></span>
<span class="line"><span>    // Validate email format</span></span>
<span class="line"><span>    if (!IsValidEmail(user[&quot;email&quot;])) {</span></span>
<span class="line"><span>      return false;</span></span>
<span class="line"><span>    }</span></span>
<span class="line"><span></span></span>
<span class="line"><span>    // Validate age range</span></span>
<span class="line"><span>    if (user[&quot;age&quot;] &lt; 0 || user[&quot;age&quot;] &gt; 150) {</span></span>
<span class="line"><span>      return false;</span></span>
<span class="line"><span>    }</span></span>
<span class="line"><span></span></span>
<span class="line"><span>    return true;</span></span>
<span class="line"><span>  }</span></span>
<span class="line"><span></span></span>
<span class="line"><span>  function ValidateArrayFixture(arr: any[], expectedType: string): boolean {</span></span>
<span class="line"><span>    if (!IsArray(arr)) {</span></span>
<span class="line"><span>      return false;</span></span>
<span class="line"><span>    }</span></span>
<span class="line"><span></span></span>
<span class="line"><span>    if (ArrayLength(arr) == 0) {</span></span>
<span class="line"><span>      return false;</span></span>
<span class="line"><span>    }</span></span>
<span class="line"><span></span></span>
<span class="line"><span>    // Check type consistency</span></span>
<span class="line"><span>    for (induce i: number = 0; i &lt; ArrayLength(arr); i = i + 1) {</span></span>
<span class="line"><span>      if (expectedType == &quot;number&quot; &amp;&amp; !IsNumber(arr[i])) {</span></span>
<span class="line"><span>        return false;</span></span>
<span class="line"><span>      }</span></span>
<span class="line"><span>      if (expectedType == &quot;string&quot; &amp;&amp; !IsString(arr[i])) {</span></span>
<span class="line"><span>        return false;</span></span>
<span class="line"><span>      }</span></span>
<span class="line"><span>    }</span></span>
<span class="line"><span></span></span>
<span class="line"><span>    return true;</span></span>
<span class="line"><span>  }</span></span>
<span class="line"><span></span></span>
<span class="line"><span>  // Test fixture validation</span></span>
<span class="line"><span>  MindLink TestData;</span></span>
<span class="line"><span></span></span>
<span class="line"><span>  Assert(ValidateUserFixture(testUser), &quot;Test user fixture should be valid&quot;);</span></span>
<span class="line"><span>  Assert(ValidateUserFixture(adminUser), &quot;Admin user fixture should be valid&quot;);</span></span>
<span class="line"><span>  Assert(ValidateArrayFixture(numberArray, &quot;number&quot;), &quot;Number array fixture should be valid&quot;);</span></span>
<span class="line"><span>  Assert(ValidateArrayFixture(stringArray, &quot;string&quot;), &quot;String array fixture should be valid&quot;);</span></span>
<span class="line"><span></span></span>
<span class="line"><span>  Observe(&quot;Fixture validation tests passed!&quot;);</span></span>
<span class="line"><span>} Relax</span></span></code></pre><div class="line-numbers-wrapper" aria-hidden="true"><span class="line-number">1</span><br><span class="line-number">2</span><br><span class="line-number">3</span><br><span class="line-number">4</span><br><span class="line-number">5</span><br><span class="line-number">6</span><br><span class="line-number">7</span><br><span class="line-number">8</span><br><span class="line-number">9</span><br><span class="line-number">10</span><br><span class="line-number">11</span><br><span class="line-number">12</span><br><span class="line-number">13</span><br><span class="line-number">14</span><br><span class="line-number">15</span><br><span class="line-number">16</span><br><span class="line-number">17</span><br><span class="line-number">18</span><br><span class="line-number">19</span><br><span class="line-number">20</span><br><span class="line-number">21</span><br><span class="line-number">22</span><br><span class="line-number">23</span><br><span class="line-number">24</span><br><span class="line-number">25</span><br><span class="line-number">26</span><br><span class="line-number">27</span><br><span class="line-number">28</span><br><span class="line-number">29</span><br><span class="line-number">30</span><br><span class="line-number">31</span><br><span class="line-number">32</span><br><span class="line-number">33</span><br><span class="line-number">34</span><br><span class="line-number">35</span><br><span class="line-number">36</span><br><span class="line-number">37</span><br><span class="line-number">38</span><br><span class="line-number">39</span><br><span class="line-number">40</span><br><span class="line-number">41</span><br><span class="line-number">42</span><br><span class="line-number">43</span><br><span class="line-number">44</span><br><span class="line-number">45</span><br><span class="line-number">46</span><br><span class="line-number">47</span><br><span class="line-number">48</span><br><span class="line-number">49</span><br><span class="line-number">50</span><br><span class="line-number">51</span><br><span class="line-number">52</span><br><span class="line-number">53</span><br><span class="line-number">54</span><br><span class="line-number">55</span><br><span class="line-number">56</span><br><span class="line-number">57</span><br><span class="line-number">58</span><br><span class="line-number">59</span><br><span class="line-number">60</span><br><span class="line-number">61</span><br></div></div><h3 id="_3-fixture-cleanup-and-reset" tabindex="-1">3. Fixture Cleanup and Reset <a class="header-anchor" href="#_3-fixture-cleanup-and-reset" aria-label="Permalink to &quot;3. Fixture Cleanup and Reset&quot;">​</a></h3><div class="language-hyp vp-adaptive-theme line-numbers-mode"><button title="Copy Code" class="copy"></button><span class="lang">hyp</span><pre class="shiki shiki-themes github-light github-dark vp-code" tabindex="0"><code><span class="line"><span>// fixture_cleanup.hyp</span></span>
<span class="line"><span>Focus {</span></span>
<span class="line"><span>  function ResetTestEnvironment(): void {</span></span>
<span class="line"><span>    // Clear any test data</span></span>
<span class="line"><span>    ClearScreen();</span></span>
<span class="line"><span>    Observe(&quot;Test environment reset&quot;);</span></span>
<span class="line"><span>  }</span></span>
<span class="line"><span></span></span>
<span class="line"><span>  function CleanupTestData(): void {</span></span>
<span class="line"><span>    // Perform cleanup operations</span></span>
<span class="line"><span>    Observe(&quot;Cleaning up test data...&quot;);</span></span>
<span class="line"><span></span></span>
<span class="line"><span>    // Reset any global state</span></span>
<span class="line"><span>    // Clear caches</span></span>
<span class="line"><span>    // Reset configurations</span></span>
<span class="line"><span></span></span>
<span class="line"><span>    Observe(&quot;Test data cleanup completed&quot;);</span></span>
<span class="line"><span>  }</span></span>
<span class="line"><span></span></span>
<span class="line"><span>  // Test with cleanup</span></span>
<span class="line"><span>  MindLink TestData;</span></span>
<span class="line"><span></span></span>
<span class="line"><span>  // Run tests</span></span>
<span class="line"><span>  induce user: record = testUser;</span></span>
<span class="line"><span>  Assert(user[&quot;name&quot;] == &quot;John Doe&quot;, &quot;User name should match fixture&quot;);</span></span>
<span class="line"><span></span></span>
<span class="line"><span>  // Cleanup after tests</span></span>
<span class="line"><span>  CleanupTestData();</span></span>
<span class="line"><span>  ResetTestEnvironment();</span></span>
<span class="line"><span></span></span>
<span class="line"><span>  Observe(&quot;Test completed with proper cleanup!&quot;);</span></span>
<span class="line"><span>} Relax</span></span></code></pre><div class="line-numbers-wrapper" aria-hidden="true"><span class="line-number">1</span><br><span class="line-number">2</span><br><span class="line-number">3</span><br><span class="line-number">4</span><br><span class="line-number">5</span><br><span class="line-number">6</span><br><span class="line-number">7</span><br><span class="line-number">8</span><br><span class="line-number">9</span><br><span class="line-number">10</span><br><span class="line-number">11</span><br><span class="line-number">12</span><br><span class="line-number">13</span><br><span class="line-number">14</span><br><span class="line-number">15</span><br><span class="line-number">16</span><br><span class="line-number">17</span><br><span class="line-number">18</span><br><span class="line-number">19</span><br><span class="line-number">20</span><br><span class="line-number">21</span><br><span class="line-number">22</span><br><span class="line-number">23</span><br><span class="line-number">24</span><br><span class="line-number">25</span><br><span class="line-number">26</span><br><span class="line-number">27</span><br><span class="line-number">28</span><br><span class="line-number">29</span><br><span class="line-number">30</span><br><span class="line-number">31</span><br><span class="line-number">32</span><br></div></div><h2 id="fixture-categories" tabindex="-1">Fixture Categories <a class="header-anchor" href="#fixture-categories" aria-label="Permalink to &quot;Fixture Categories&quot;">​</a></h2><h3 id="_1-data-fixtures" tabindex="-1">1. Data Fixtures <a class="header-anchor" href="#_1-data-fixtures" aria-label="Permalink to &quot;1. Data Fixtures&quot;">​</a></h3><div class="language-hyp vp-adaptive-theme line-numbers-mode"><button title="Copy Code" class="copy"></button><span class="lang">hyp</span><pre class="shiki shiki-themes github-light github-dark vp-code" tabindex="0"><code><span class="line"><span>// data_fixtures.hyp</span></span>
<span class="line"><span>Session DataFixtures {</span></span>
<span class="line"><span>  // User data</span></span>
<span class="line"><span>  induce users: record[] = [</span></span>
<span class="line"><span>    {&quot;id&quot;: 1, &quot;name&quot;: &quot;Alice&quot;, &quot;email&quot;: &quot;alice@example.com&quot;},</span></span>
<span class="line"><span>    {&quot;id&quot;: 2, &quot;name&quot;: &quot;Bob&quot;, &quot;email&quot;: &quot;bob@example.com&quot;},</span></span>
<span class="line"><span>    {&quot;id&quot;: 3, &quot;name&quot;: &quot;Charlie&quot;, &quot;email&quot;: &quot;charlie@example.com&quot;}</span></span>
<span class="line"><span>  ];</span></span>
<span class="line"><span></span></span>
<span class="line"><span>  // Product data</span></span>
<span class="line"><span>  induce products: record[] = [</span></span>
<span class="line"><span>    {&quot;id&quot;: &quot;P001&quot;, &quot;name&quot;: &quot;Laptop&quot;, &quot;price&quot;: 999.99, &quot;category&quot;: &quot;Electronics&quot;},</span></span>
<span class="line"><span>    {&quot;id&quot;: &quot;P002&quot;, &quot;name&quot;: &quot;Book&quot;, &quot;price&quot;: 19.99, &quot;category&quot;: &quot;Books&quot;},</span></span>
<span class="line"><span>    {&quot;id&quot;: &quot;P003&quot;, &quot;name&quot;: &quot;Coffee&quot;, &quot;price&quot;: 4.99, &quot;category&quot;: &quot;Food&quot;}</span></span>
<span class="line"><span>  ];</span></span>
<span class="line"><span></span></span>
<span class="line"><span>  // Configuration data</span></span>
<span class="line"><span>  induce settings: record = {</span></span>
<span class="line"><span>    &quot;theme&quot;: &quot;dark&quot;,</span></span>
<span class="line"><span>    &quot;language&quot;: &quot;en&quot;,</span></span>
<span class="line"><span>    &quot;timezone&quot;: &quot;UTC&quot;,</span></span>
<span class="line"><span>    &quot;notifications&quot;: true</span></span>
<span class="line"><span>  };</span></span>
<span class="line"><span>}</span></span></code></pre><div class="line-numbers-wrapper" aria-hidden="true"><span class="line-number">1</span><br><span class="line-number">2</span><br><span class="line-number">3</span><br><span class="line-number">4</span><br><span class="line-number">5</span><br><span class="line-number">6</span><br><span class="line-number">7</span><br><span class="line-number">8</span><br><span class="line-number">9</span><br><span class="line-number">10</span><br><span class="line-number">11</span><br><span class="line-number">12</span><br><span class="line-number">13</span><br><span class="line-number">14</span><br><span class="line-number">15</span><br><span class="line-number">16</span><br><span class="line-number">17</span><br><span class="line-number">18</span><br><span class="line-number">19</span><br><span class="line-number">20</span><br><span class="line-number">21</span><br><span class="line-number">22</span><br><span class="line-number">23</span><br><span class="line-number">24</span><br></div></div><h3 id="_2-state-fixtures" tabindex="-1">2. State Fixtures <a class="header-anchor" href="#_2-state-fixtures" aria-label="Permalink to &quot;2. State Fixtures&quot;">​</a></h3><div class="language-hyp vp-adaptive-theme line-numbers-mode"><button title="Copy Code" class="copy"></button><span class="lang">hyp</span><pre class="shiki shiki-themes github-light github-dark vp-code" tabindex="0"><code><span class="line"><span>// state_fixtures.hyp</span></span>
<span class="line"><span>Session StateFixtures {</span></span>
<span class="line"><span>  // Application state</span></span>
<span class="line"><span>  induce appState: record = {</span></span>
<span class="line"><span>    &quot;isLoggedIn&quot;: true,</span></span>
<span class="line"><span>    &quot;currentUser&quot;: &quot;admin&quot;,</span></span>
<span class="line"><span>    &quot;permissions&quot;: [&quot;read&quot;, &quot;write&quot;, &quot;delete&quot;],</span></span>
<span class="line"><span>    &quot;sessionTimeout&quot;: 3600</span></span>
<span class="line"><span>  };</span></span>
<span class="line"><span></span></span>
<span class="line"><span>  // Form state</span></span>
<span class="line"><span>  induce formState: record = {</span></span>
<span class="line"><span>    &quot;isValid&quot;: true,</span></span>
<span class="line"><span>    &quot;isSubmitted&quot;: false,</span></span>
<span class="line"><span>    &quot;errors&quot;: [],</span></span>
<span class="line"><span>    &quot;values&quot;: {</span></span>
<span class="line"><span>      &quot;username&quot;: &quot;testuser&quot;,</span></span>
<span class="line"><span>      &quot;email&quot;: &quot;test@example.com&quot;,</span></span>
<span class="line"><span>      &quot;password&quot;: &quot;********&quot;</span></span>
<span class="line"><span>    }</span></span>
<span class="line"><span>  };</span></span>
<span class="line"><span>}</span></span></code></pre><div class="line-numbers-wrapper" aria-hidden="true"><span class="line-number">1</span><br><span class="line-number">2</span><br><span class="line-number">3</span><br><span class="line-number">4</span><br><span class="line-number">5</span><br><span class="line-number">6</span><br><span class="line-number">7</span><br><span class="line-number">8</span><br><span class="line-number">9</span><br><span class="line-number">10</span><br><span class="line-number">11</span><br><span class="line-number">12</span><br><span class="line-number">13</span><br><span class="line-number">14</span><br><span class="line-number">15</span><br><span class="line-number">16</span><br><span class="line-number">17</span><br><span class="line-number">18</span><br><span class="line-number">19</span><br><span class="line-number">20</span><br><span class="line-number">21</span><br><span class="line-number">22</span><br></div></div><h3 id="_3-error-fixtures" tabindex="-1">3. Error Fixtures <a class="header-anchor" href="#_3-error-fixtures" aria-label="Permalink to &quot;3. Error Fixtures&quot;">​</a></h3><div class="language-hyp vp-adaptive-theme line-numbers-mode"><button title="Copy Code" class="copy"></button><span class="lang">hyp</span><pre class="shiki shiki-themes github-light github-dark vp-code" tabindex="0"><code><span class="line"><span>// error_fixtures.hyp</span></span>
<span class="line"><span>Session ErrorFixtures {</span></span>
<span class="line"><span>  // Common error scenarios</span></span>
<span class="line"><span>  induce validationErrors: record[] = [</span></span>
<span class="line"><span>    {&quot;field&quot;: &quot;email&quot;, &quot;message&quot;: &quot;Invalid email format&quot;, &quot;code&quot;: &quot;EMAIL_INVALID&quot;},</span></span>
<span class="line"><span>    {&quot;field&quot;: &quot;password&quot;, &quot;message&quot;: &quot;Password too short&quot;, &quot;code&quot;: &quot;PASSWORD_SHORT&quot;},</span></span>
<span class="line"><span>    {&quot;field&quot;: &quot;age&quot;, &quot;message&quot;: &quot;Age must be positive&quot;, &quot;code&quot;: &quot;AGE_INVALID&quot;}</span></span>
<span class="line"><span>  ];</span></span>
<span class="line"><span></span></span>
<span class="line"><span>  induce networkErrors: record[] = [</span></span>
<span class="line"><span>    {&quot;code&quot;: 404, &quot;message&quot;: &quot;Resource not found&quot;, &quot;type&quot;: &quot;NOT_FOUND&quot;},</span></span>
<span class="line"><span>    {&quot;code&quot;: 500, &quot;message&quot;: &quot;Internal server error&quot;, &quot;type&quot;: &quot;SERVER_ERROR&quot;},</span></span>
<span class="line"><span>    {&quot;code&quot;: 403, &quot;message&quot;: &quot;Access forbidden&quot;, &quot;type&quot;: &quot;FORBIDDEN&quot;}</span></span>
<span class="line"><span>  ];</span></span>
<span class="line"><span>}</span></span></code></pre><div class="line-numbers-wrapper" aria-hidden="true"><span class="line-number">1</span><br><span class="line-number">2</span><br><span class="line-number">3</span><br><span class="line-number">4</span><br><span class="line-number">5</span><br><span class="line-number">6</span><br><span class="line-number">7</span><br><span class="line-number">8</span><br><span class="line-number">9</span><br><span class="line-number">10</span><br><span class="line-number">11</span><br><span class="line-number">12</span><br><span class="line-number">13</span><br><span class="line-number">14</span><br><span class="line-number">15</span><br></div></div><h2 id="best-practices" tabindex="-1">Best Practices <a class="header-anchor" href="#best-practices" aria-label="Permalink to &quot;Best Practices&quot;">​</a></h2><h3 id="_1-fixture-organization" tabindex="-1">1. Fixture Organization <a class="header-anchor" href="#_1-fixture-organization" aria-label="Permalink to &quot;1. Fixture Organization&quot;">​</a></h3><div class="language-hyp vp-adaptive-theme line-numbers-mode"><button title="Copy Code" class="copy"></button><span class="lang">hyp</span><pre class="shiki shiki-themes github-light github-dark vp-code" tabindex="0"><code><span class="line"><span>// Organize fixtures by domain</span></span>
<span class="line"><span>Session UserFixtures {</span></span>
<span class="line"><span>  // User-related test data</span></span>
<span class="line"><span>}</span></span>
<span class="line"><span></span></span>
<span class="line"><span>Session ProductFixtures {</span></span>
<span class="line"><span>  // Product-related test data</span></span>
<span class="line"><span>}</span></span>
<span class="line"><span></span></span>
<span class="line"><span>Session ConfigFixtures {</span></span>
<span class="line"><span>  // Configuration test data</span></span>
<span class="line"><span>}</span></span></code></pre><div class="line-numbers-wrapper" aria-hidden="true"><span class="line-number">1</span><br><span class="line-number">2</span><br><span class="line-number">3</span><br><span class="line-number">4</span><br><span class="line-number">5</span><br><span class="line-number">6</span><br><span class="line-number">7</span><br><span class="line-number">8</span><br><span class="line-number">9</span><br><span class="line-number">10</span><br><span class="line-number">11</span><br><span class="line-number">12</span><br></div></div><h3 id="_2-fixture-naming-conventions" tabindex="-1">2. Fixture Naming Conventions <a class="header-anchor" href="#_2-fixture-naming-conventions" aria-label="Permalink to &quot;2. Fixture Naming Conventions&quot;">​</a></h3><div class="language-hyp vp-adaptive-theme line-numbers-mode"><button title="Copy Code" class="copy"></button><span class="lang">hyp</span><pre class="shiki shiki-themes github-light github-dark vp-code" tabindex="0"><code><span class="line"><span>// Use descriptive names</span></span>
<span class="line"><span>induce validUserFixture: record = {...};</span></span>
<span class="line"><span>induce invalidUserFixture: record = {...};</span></span>
<span class="line"><span>induce adminUserFixture: record = {...};</span></span>
<span class="line"><span></span></span>
<span class="line"><span>// Use consistent naming patterns</span></span>
<span class="line"><span>induce testData_Users: record[] = {...};</span></span>
<span class="line"><span>induce testData_Products: record[] = {...};</span></span>
<span class="line"><span>induce testData_Config: record = {...};</span></span></code></pre><div class="line-numbers-wrapper" aria-hidden="true"><span class="line-number">1</span><br><span class="line-number">2</span><br><span class="line-number">3</span><br><span class="line-number">4</span><br><span class="line-number">5</span><br><span class="line-number">6</span><br><span class="line-number">7</span><br><span class="line-number">8</span><br><span class="line-number">9</span><br></div></div><h3 id="_3-fixture-documentation" tabindex="-1">3. Fixture Documentation <a class="header-anchor" href="#_3-fixture-documentation" aria-label="Permalink to &quot;3. Fixture Documentation&quot;">​</a></h3><div class="language-hyp vp-adaptive-theme line-numbers-mode"><button title="Copy Code" class="copy"></button><span class="lang">hyp</span><pre class="shiki shiki-themes github-light github-dark vp-code" tabindex="0"><code><span class="line"><span>// Document your fixtures</span></span>
<span class="line"><span>Session WellDocumentedFixtures {</span></span>
<span class="line"><span>  // User fixture for testing authentication</span></span>
<span class="line"><span>  // Contains valid user credentials and profile data</span></span>
<span class="line"><span>  induce testUser: record = {</span></span>
<span class="line"><span>    &quot;username&quot;: &quot;testuser&quot;,</span></span>
<span class="line"><span>    &quot;password&quot;: &quot;testpass123&quot;,</span></span>
<span class="line"><span>    &quot;email&quot;: &quot;test@example.com&quot;,</span></span>
<span class="line"><span>    &quot;profile&quot;: {</span></span>
<span class="line"><span>      &quot;firstName&quot;: &quot;Test&quot;,</span></span>
<span class="line"><span>      &quot;lastName&quot;: &quot;User&quot;,</span></span>
<span class="line"><span>      &quot;age&quot;: 25</span></span>
<span class="line"><span>    }</span></span>
<span class="line"><span>  };</span></span>
<span class="line"><span></span></span>
<span class="line"><span>  // Admin user fixture for testing authorization</span></span>
<span class="line"><span>  // Contains admin privileges and elevated permissions</span></span>
<span class="line"><span>  induce adminUser: record = {</span></span>
<span class="line"><span>    &quot;username&quot;: &quot;admin&quot;,</span></span>
<span class="line"><span>    &quot;password&quot;: &quot;adminpass123&quot;,</span></span>
<span class="line"><span>    &quot;email&quot;: &quot;admin@example.com&quot;,</span></span>
<span class="line"><span>    &quot;role&quot;: &quot;admin&quot;,</span></span>
<span class="line"><span>    &quot;permissions&quot;: [&quot;read&quot;, &quot;write&quot;, &quot;delete&quot;, &quot;admin&quot;]</span></span>
<span class="line"><span>  };</span></span>
<span class="line"><span>}</span></span></code></pre><div class="line-numbers-wrapper" aria-hidden="true"><span class="line-number">1</span><br><span class="line-number">2</span><br><span class="line-number">3</span><br><span class="line-number">4</span><br><span class="line-number">5</span><br><span class="line-number">6</span><br><span class="line-number">7</span><br><span class="line-number">8</span><br><span class="line-number">9</span><br><span class="line-number">10</span><br><span class="line-number">11</span><br><span class="line-number">12</span><br><span class="line-number">13</span><br><span class="line-number">14</span><br><span class="line-number">15</span><br><span class="line-number">16</span><br><span class="line-number">17</span><br><span class="line-number">18</span><br><span class="line-number">19</span><br><span class="line-number">20</span><br><span class="line-number">21</span><br><span class="line-number">22</span><br><span class="line-number">23</span><br><span class="line-number">24</span><br><span class="line-number">25</span><br></div></div><h3 id="_4-fixture-reusability" tabindex="-1">4. Fixture Reusability <a class="header-anchor" href="#_4-fixture-reusability" aria-label="Permalink to &quot;4. Fixture Reusability&quot;">​</a></h3><div class="language-hyp vp-adaptive-theme line-numbers-mode"><button title="Copy Code" class="copy"></button><span class="lang">hyp</span><pre class="shiki shiki-themes github-light github-dark vp-code" tabindex="0"><code><span class="line"><span>// Create reusable fixture components</span></span>
<span class="line"><span>function CreateBaseUser(name: string, email: string): record {</span></span>
<span class="line"><span>  return {</span></span>
<span class="line"><span>    &quot;name&quot;: name,</span></span>
<span class="line"><span>    &quot;email&quot;: email,</span></span>
<span class="line"><span>    &quot;createdAt&quot;: GetCurrentTime(),</span></span>
<span class="line"><span>    &quot;isActive&quot;: true</span></span>
<span class="line"><span>  };</span></span>
<span class="line"><span>}</span></span>
<span class="line"><span></span></span>
<span class="line"><span>function CreateUserWithRole(name: string, email: string, role: string): record {</span></span>
<span class="line"><span>  induce baseUser: record = CreateBaseUser(name, email);</span></span>
<span class="line"><span>  baseUser[&quot;role&quot;] = role;</span></span>
<span class="line"><span>  return baseUser;</span></span>
<span class="line"><span>}</span></span></code></pre><div class="line-numbers-wrapper" aria-hidden="true"><span class="line-number">1</span><br><span class="line-number">2</span><br><span class="line-number">3</span><br><span class="line-number">4</span><br><span class="line-number">5</span><br><span class="line-number">6</span><br><span class="line-number">7</span><br><span class="line-number">8</span><br><span class="line-number">9</span><br><span class="line-number">10</span><br><span class="line-number">11</span><br><span class="line-number">12</span><br><span class="line-number">13</span><br><span class="line-number">14</span><br><span class="line-number">15</span><br></div></div><h2 id="integration-with-test-framework" tabindex="-1">Integration with Test Framework <a class="header-anchor" href="#integration-with-test-framework" aria-label="Permalink to &quot;Integration with Test Framework&quot;">​</a></h2><h3 id="_1-using-fixtures-in-test-commands" tabindex="-1">1. Using Fixtures in Test Commands <a class="header-anchor" href="#_1-using-fixtures-in-test-commands" aria-label="Permalink to &quot;1. Using Fixtures in Test Commands&quot;">​</a></h3><div class="language-bash vp-adaptive-theme line-numbers-mode"><button title="Copy Code" class="copy"></button><span class="lang">bash</span><pre class="shiki shiki-themes github-light github-dark vp-code" tabindex="0"><code><span class="line"><span style="--shiki-light:#6A737D;--shiki-dark:#6A737D;"># Run tests with specific fixtures</span></span>
<span class="line"><span style="--shiki-light:#6F42C1;--shiki-dark:#B392F0;">dotnet</span><span style="--shiki-light:#032F62;--shiki-dark:#9ECBFF;"> run</span><span style="--shiki-light:#005CC5;--shiki-dark:#79B8FF;"> --</span><span style="--shiki-light:#032F62;--shiki-dark:#9ECBFF;"> test</span><span style="--shiki-light:#032F62;--shiki-dark:#9ECBFF;"> test_with_fixtures.hyp</span><span style="--shiki-light:#005CC5;--shiki-dark:#79B8FF;"> --verbose</span></span>
<span class="line"></span>
<span class="line"><span style="--shiki-light:#6A737D;--shiki-dark:#6A737D;"># Run tests with fixture validation</span></span>
<span class="line"><span style="--shiki-light:#6F42C1;--shiki-dark:#B392F0;">dotnet</span><span style="--shiki-light:#032F62;--shiki-dark:#9ECBFF;"> run</span><span style="--shiki-light:#005CC5;--shiki-dark:#79B8FF;"> --</span><span style="--shiki-light:#032F62;--shiki-dark:#9ECBFF;"> test</span><span style="--shiki-light:#032F62;--shiki-dark:#9ECBFF;"> fixture_validation.hyp</span><span style="--shiki-light:#005CC5;--shiki-dark:#79B8FF;"> --debug</span></span></code></pre><div class="line-numbers-wrapper" aria-hidden="true"><span class="line-number">1</span><br><span class="line-number">2</span><br><span class="line-number">3</span><br><span class="line-number">4</span><br><span class="line-number">5</span><br></div></div><h3 id="_2-fixture-loading-in-tests" tabindex="-1">2. Fixture Loading in Tests <a class="header-anchor" href="#_2-fixture-loading-in-tests" aria-label="Permalink to &quot;2. Fixture Loading in Tests&quot;">​</a></h3><div class="language-hyp vp-adaptive-theme line-numbers-mode"><button title="Copy Code" class="copy"></button><span class="lang">hyp</span><pre class="shiki shiki-themes github-light github-dark vp-code" tabindex="0"><code><span class="line"><span>// test_integration.hyp</span></span>
<span class="line"><span>Focus {</span></span>
<span class="line"><span>  // Load multiple fixture sessions</span></span>
<span class="line"><span>  MindLink TestData;</span></span>
<span class="line"><span>  MindLink DataFixtures;</span></span>
<span class="line"><span>  MindLink ErrorFixtures;</span></span>
<span class="line"><span></span></span>
<span class="line"><span>  // Test with combined fixtures</span></span>
<span class="line"><span>  induce user: record = testUser;</span></span>
<span class="line"><span>  induce products: record[] = products;</span></span>
<span class="line"><span>  induce errors: record[] = validationErrors;</span></span>
<span class="line"><span></span></span>
<span class="line"><span>  // Comprehensive testing</span></span>
<span class="line"><span>  Assert(ValidateUserFixture(user), &quot;User fixture should be valid&quot;);</span></span>
<span class="line"><span>  Assert(ArrayLength(products) &gt; 0, &quot;Products fixture should not be empty&quot;);</span></span>
<span class="line"><span>  Assert(ArrayLength(errors) &gt; 0, &quot;Error fixtures should be available&quot;);</span></span>
<span class="line"><span></span></span>
<span class="line"><span>  Observe(&quot;Integration test with fixtures completed successfully!&quot;);</span></span>
<span class="line"><span>} Relax</span></span></code></pre><div class="line-numbers-wrapper" aria-hidden="true"><span class="line-number">1</span><br><span class="line-number">2</span><br><span class="line-number">3</span><br><span class="line-number">4</span><br><span class="line-number">5</span><br><span class="line-number">6</span><br><span class="line-number">7</span><br><span class="line-number">8</span><br><span class="line-number">9</span><br><span class="line-number">10</span><br><span class="line-number">11</span><br><span class="line-number">12</span><br><span class="line-number">13</span><br><span class="line-number">14</span><br><span class="line-number">15</span><br><span class="line-number">16</span><br><span class="line-number">17</span><br><span class="line-number">18</span><br><span class="line-number">19</span><br></div></div><h2 id="conclusion" tabindex="-1">Conclusion <a class="header-anchor" href="#conclusion" aria-label="Permalink to &quot;Conclusion&quot;">​</a></h2><p>Test fixtures are essential for creating reliable, maintainable tests in HypnoScript. By following these patterns and best practices, you can create comprehensive test suites that are easy to understand, maintain, and extend.</p><p>Remember to:</p><ul><li>Keep fixtures simple and focused</li><li>Use descriptive names and documentation</li><li>Validate fixture data</li><li>Organize fixtures logically</li><li>Reuse fixture components when possible</li><li>Clean up after tests</li></ul><p>This approach will help you build robust test suites that catch issues early and provide confidence in your code quality.</p>`,42)])])}const d=n(l,[["render",i]]);export{m as __pageData,d as default};
