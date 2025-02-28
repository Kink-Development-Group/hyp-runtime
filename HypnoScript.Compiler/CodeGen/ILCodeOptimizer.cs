using System;
using System.Collections.Generic;
using System.Reflection.Emit;

namespace HypnoScript.Compiler.CodeGen
{
	// Enterprise-Level: Definition einer einfachen Intermediate Representation (IR) für IL-Anweisungen.
	public class IlInstruction
	{
		public OpCode Opcode { get; set; }
		public object? Operand { get; set; }

		public IlInstruction(OpCode opcode, object? operand = null)
		{
			Opcode = opcode;
			Operand = operand;
		}

		public override string ToString() => Operand != null
			? $"{Opcode.Name} {Operand}"
			: (Opcode.Name ?? string.Empty);
	}

	public static class ILCodeOptimizer
	{
		// Enterprise-Level: Optimiert den IL-Code, indem überflüssige Box/Unbox-Aufrufe entfernt werden.
		// Diese Methode arbeitet anhand einer Liste von IlInstruction und gibt eine optimierte Liste zurück.
		public static List<IlInstruction> Optimize(List<IlInstruction> instructions)
		{
			var optimized = new List<IlInstruction>();
			int i = 0;
			while (i < instructions.Count)
			{
				// Prüfe auf Box/Unbox-Paare, die sich gegenseitig aufheben:
				if (i < instructions.Count - 1 &&
					IsBoxInstruction(instructions[i]) &&
					IsUnboxInstruction(instructions[i + 1]) &&
					MatchingTypes(instructions[i], instructions[i + 1]))
				{
					// Diese beiden Anweisungen heben sich auf – überspringe sie
					i += 2;
					continue;
				}
				optimized.Add(instructions[i]);
				i++;
			}
			return optimized;
		}

		private static bool IsBoxInstruction(IlInstruction instr) =>
			instr.Opcode == OpCodes.Box;

		private static bool IsUnboxInstruction(IlInstruction instr) =>
			instr.Opcode == OpCodes.Unbox_Any || instr.Opcode == OpCodes.Unbox;

		// Überprüft, ob die Box/Unbox-Paare denselben Typ betreffen.
		private static bool MatchingTypes(IlInstruction boxInstr, IlInstruction unboxInstr)
		{
			if (boxInstr.Operand is Type boxType && unboxInstr.Operand is Type unboxType)
			{
				return boxType == unboxType;
			}
			return false;
		}

		// Optional: Eine Methode zur Ausgabe der IR-Instruktionen (zur Diagnose)
		public static void DumpInstructions(List<IlInstruction> instructions)
		{
			Console.WriteLine("Optimized IL Instructions:");
			foreach (var instr in instructions)
			{
				Console.WriteLine(instr.ToString());
			}
		}
	}
}
