using System;
using System.Collections.Generic;

namespace HypnoScript.Runtime.Builtins
{
    /// <summary>
    /// Stellt hypnotische und trancebezogene Funktionen für HypnoScript bereit.
    /// </summary>
    public static class HypnoticBuiltins
    {
        /// <summary>
        /// Enters a deep trance state for the specified duration.
        /// </summary>
        /// <param name="duration">Duration in milliseconds (default: 5000)</param>
        public static void DeepTrance(int duration = 5000)
        {
            HypnoBuiltins.Observe("Entering deep trance...");
            HypnoBuiltins.Drift(duration);
            HypnoBuiltins.Observe("Emerging from trance...");
        }

        /// <summary>
        /// Performs a hypnotic countdown from the specified number.
        /// </summary>
        /// <param name="from">Starting number for countdown (default: 10)</param>
        public static void HypnoticCountdown(int from = 10)
        {
            for (int i = from; i > 0; i--)
            {
                HypnoBuiltins.Observe($"You are feeling very sleepy... {i}");
                HypnoBuiltins.Drift(1000);
            }
            HypnoBuiltins.Observe("You are now in a deep hypnotic state.");
        }

        /// <summary>
        /// Performs a trance induction for the specified subject.
        /// </summary>
        /// <param name="subjectName">Name of the subject (default: "Subject")</param>
        public static void TranceInduction(string subjectName = "Subject")
        {
            HypnoBuiltins.Observe($"Welcome {subjectName}, you are about to enter a deep trance...");
            HypnoBuiltins.Drift(2000);
            HypnoBuiltins.Observe("Take a deep breath and relax...");
            HypnoBuiltins.Drift(1500);
            HypnoBuiltins.Observe("With each breath, you feel more and more relaxed...");
            HypnoBuiltins.Drift(1500);
            HypnoBuiltins.Observe("Your mind is becoming clear and focused...");
            HypnoBuiltins.Drift(1000);
        }

        /// <summary>
        /// Guides the subject through hypnotic visualization.
        /// </summary>
        /// <param name="scene">Scene to visualize (default: "a peaceful garden")</param>
        public static void HypnoticVisualization(string scene = "a peaceful garden")
        {
            HypnoBuiltins.Observe($"Imagine yourself in {scene}...");
            HypnoBuiltins.Drift(2000);
            HypnoBuiltins.Observe("Feel the tranquility surrounding you...");
            HypnoBuiltins.Drift(1500);
            HypnoBuiltins.Observe("Every detail becomes clearer and more vivid...");
            HypnoBuiltins.Drift(1500);
        }

        /// <summary>
        /// Performs progressive relaxation with the specified number of steps.
        /// </summary>
        /// <param name="steps">Number of relaxation steps (default: 5)</param>
        public static void ProgressiveRelaxation(int steps = 5)
        {
            HypnoBuiltins.Observe("Let's begin progressive relaxation...");
            for (int i = 1; i <= steps; i++)
            {
                HypnoBuiltins.Observe($"Step {i}: Relax your muscles deeper and deeper...");
                HypnoBuiltins.Drift(1500);
            }
            HypnoBuiltins.Observe("You are now completely relaxed and at peace.");
        }

        /// <summary>
        /// Gives a hypnotic suggestion to the subject.
        /// </summary>
        /// <param name="suggestion">The suggestion to implant</param>
        public static void HypnoticSuggestion(string suggestion)
        {
            HypnoBuiltins.Observe("I will now give you a powerful suggestion...");
            HypnoBuiltins.Drift(1000);
            HypnoBuiltins.Observe($"Remember this: {suggestion}");
            HypnoBuiltins.Drift(2000);
            HypnoBuiltins.Observe("This suggestion will become stronger with each passing moment.");
        }

        /// <summary>
        /// Deepens the trance state by the specified number of levels.
        /// </summary>
        /// <param name="levels">Number of deepening levels (default: 3)</param>
        public static void TranceDeepening(int levels = 3)
        {
            HypnoBuiltins.Observe("We will now go deeper into trance...");
            for (int i = 1; i <= levels; i++)
            {
                HypnoBuiltins.Observe($"Level {i}: Going deeper...");
                HypnoBuiltins.Drift(2000);
            }
            HypnoBuiltins.Observe("You are now in the deepest level of trance.");
        }

        /// <summary>
        /// Guides the subject through hypnotic breathing exercises.
        /// </summary>
        /// <param name="cycles">Number of breathing cycles (default: 5)</param>
        public static void HypnoticBreathing(int cycles = 5)
        {
            HypnoBuiltins.Observe("Let's practice hypnotic breathing...");
            for (int i = 1; i <= cycles; i++)
            {
                HypnoBuiltins.Observe($"Cycle {i}: Breathe in deeply...");
                HypnoBuiltins.Drift(2000);
                HypnoBuiltins.Observe("Hold your breath...");
                HypnoBuiltins.Drift(1000);
                HypnoBuiltins.Observe("Now exhale slowly...");
                HypnoBuiltins.Drift(2000);
            }
            HypnoBuiltins.Observe("You are now in a state of perfect calm.");
        }

        /// <summary>
        /// Creates a hypnotic anchor for the specified state.
        /// </summary>
        /// <param name="anchor">The anchor state to create (default: "peaceful")</param>
        public static void HypnoticAnchoring(string anchor = "peaceful")
        {
            HypnoBuiltins.Observe($"I will now create a powerful anchor for '{anchor}'...");
            HypnoBuiltins.Drift(1500);
            HypnoBuiltins.Observe("Every time you think of this anchor, you will feel this way...");
            HypnoBuiltins.Drift(2000);
            HypnoBuiltins.Observe($"Your '{anchor}' anchor is now established.");
        }

        /// <summary>
        /// Performs hypnotic age regression to the specified age.
        /// </summary>
        /// <param name="age">Target age for regression (default: 10)</param>
        public static void HypnoticRegression(int age = 10)
        {
            HypnoBuiltins.Observe($"We will now travel back in time to when you were {age} years old...");
            HypnoBuiltins.Drift(3000);
            HypnoBuiltins.Observe("You can see yourself as a child...");
            HypnoBuiltins.Drift(2000);
            HypnoBuiltins.Observe("Feel the memories and emotions of that time...");
            HypnoBuiltins.Drift(2000);
            HypnoBuiltins.Observe("You are now experiencing your past self.");
        }

        /// <summary>
        /// Performs hypnotic future progression to the specified number of years ahead.
        /// </summary>
        /// <param name="years">Number of years into the future (default: 5)</param>
        public static void HypnoticFutureProgression(int years = 5)
        {
            HypnoBuiltins.Observe($"Let's travel forward {years} years into your future...");
            HypnoBuiltins.Drift(3000);
            HypnoBuiltins.Observe("You can see your future self...");
            HypnoBuiltins.Drift(2000);
            HypnoBuiltins.Observe("Feel the wisdom and experience of your future...");
            HypnoBuiltins.Drift(2000);
            HypnoBuiltins.Observe("You are now connected to your future potential.");
        }

        /// <summary>
        /// Establishes a pattern matching system for the specified pattern.
        /// </summary>
        /// <param name="pattern">The pattern to establish</param>
        public static void HypnoticPatternMatching(string pattern)
        {
            HypnoBuiltins.Observe($"I will now establish a pattern matching system for '{pattern}'...");
            HypnoBuiltins.Drift(2000);
            HypnoBuiltins.Observe("Your mind will automatically recognize this pattern...");
            HypnoBuiltins.Drift(1500);
            HypnoBuiltins.Observe("Every time you encounter this pattern, you will respond automatically...");
            HypnoBuiltins.Drift(2000);
            HypnoBuiltins.Observe($"The '{pattern}' pattern is now deeply embedded in your subconscious.");
        }

        /// <summary>
        /// Alters the subject's perception of time by the specified factor.
        /// </summary>
        /// <param name="factor">Time dilation factor (default: 2.0)</param>
        public static void HypnoticTimeDilation(double factor = 2.0)
        {
            HypnoBuiltins.Observe($"I will now alter your perception of time by a factor of {factor}...");
            HypnoBuiltins.Drift(3000);
            HypnoBuiltins.Observe("Time will feel different to you now...");
            HypnoBuiltins.Drift(2000);
            HypnoBuiltins.Observe("Minutes will feel like hours, or hours like minutes...");
            HypnoBuiltins.Drift(2000);
            HypnoBuiltins.Observe("Your time perception has been successfully modified.");
        }

        /// <summary>
        /// Enhances the subject's memory capabilities.
        /// </summary>
        public static void HypnoticMemoryEnhancement()
        {
            HypnoBuiltins.Observe("I will now enhance your memory capabilities...");
            HypnoBuiltins.Drift(2000);
            HypnoBuiltins.Observe("Your ability to remember and recall information will improve...");
            HypnoBuiltins.Drift(2000);
            HypnoBuiltins.Observe("You will find it easier to learn and retain new knowledge...");
            HypnoBuiltins.Drift(2000);
            HypnoBuiltins.Observe("Your memory enhancement is now active.");
        }

        /// <summary>
        /// Boosts the subject's creative potential.
        /// </summary>
        public static void HypnoticCreativityBoost()
        {
            HypnoBuiltins.Observe("I will now unlock your creative potential...");
            HypnoBuiltins.Drift(2000);
            HypnoBuiltins.Observe("Your imagination will become more vivid and active...");
            HypnoBuiltins.Drift(2000);
            HypnoBuiltins.Observe("Creative solutions will come to you more easily...");
            HypnoBuiltins.Drift(2000);
            HypnoBuiltins.Observe("Your creativity is now enhanced.");
        }
    }
}
