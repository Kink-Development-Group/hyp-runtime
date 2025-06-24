using System;
using System.Globalization;

namespace HypnoScript.Runtime.Builtins
{
	/// <summary>
	/// Stellt Zeit- und Datumsfunktionen für HypnoScript bereit.
	/// </summary>
	public static class TimeBuiltins
	{
		/// <summary>
		/// Gets current Unix timestamp
		/// </summary>
		public static int GetCurrentTime() => (int)DateTimeOffset.UtcNow.ToUnixTimeSeconds();

		/// <summary>
		/// Gets current date as string
		/// </summary>
		public static string GetCurrentDate() => DateTime.Now.ToString("yyyy-MM-dd");

		/// <summary>
		/// Gets current time as string
		/// </summary>
		public static string GetCurrentTimeString() => DateTime.Now.ToString("HH:mm:ss");

		/// <summary>
		/// Gets current date and time as string
		/// </summary>
		public static string GetCurrentDateTime() => DateTime.Now.ToString("yyyy-MM-dd HH:mm:ss");

		/// <summary>
		/// Formats date time with custom format
		/// </summary>
		public static string FormatDateTime(string format = "yyyy-MM-dd HH:mm:ss")
		{
			return DateTime.Now.ToString(format);
		}

		/// <summary>
		/// Gets day of week (0=Sunday, 6=Saturday)
		/// </summary>
		public static int GetDayOfWeek() => (int)DateTime.Now.DayOfWeek;

		/// <summary>
		/// Gets day of year
		/// </summary>
		public static int GetDayOfYear() => DateTime.Now.DayOfYear;

		/// <summary>
		/// Checks if year is leap year
		/// </summary>
		public static bool IsLeapYear(int year) => DateTime.IsLeapYear(year);

		/// <summary>
		/// Gets number of days in month
		/// </summary>
		public static int GetDaysInMonth(int year, int month) => DateTime.DaysInMonth(year, month);

		/// <summary>
		/// Gets timezone information
		/// </summary>
		public static string GetTimeZone() => TimeZoneInfo.Local.DisplayName;

		/// <summary>
		/// Converts time between timezones
		/// </summary>
		public static string ConvertTimeZone(string date, string fromZone, string toZone)
		{
			try
			{
				var fromTz = TimeZoneInfo.FindSystemTimeZoneById(fromZone);
				var toTz = TimeZoneInfo.FindSystemTimeZoneById(toZone);
				var dt = DateTime.Parse(date);
				var utc = TimeZoneInfo.ConvertTimeToUtc(dt, fromTz);
				var converted = TimeZoneInfo.ConvertTimeFromUtc(utc, toTz);
				return converted.ToString("yyyy-MM-dd HH:mm:ss");
			}
			catch
			{
				return date;
			}
		}

		/// <summary>
		/// Gets week of year
		/// </summary>
		public static int GetWeekOfYear(string date)
		{
			var dt = DateTime.Parse(date);
			var calendar = CultureInfo.InvariantCulture.Calendar;
			return calendar.GetWeekOfYear(dt, CalendarWeekRule.FirstFourDayWeek, DayOfWeek.Monday);
		}

		/// <summary>
		/// Gets quarter of year
		/// </summary>
		public static int GetQuarter(string date)
		{
			var dt = DateTime.Parse(date);
			return (dt.Month - 1) / 3 + 1;
		}

		/// <summary>
		/// Checks if date is weekend
		/// </summary>
		public static bool IsWeekend(string date)
		{
			var dt = DateTime.Parse(date);
			return dt.DayOfWeek == DayOfWeek.Saturday || dt.DayOfWeek == DayOfWeek.Sunday;
		}

		/// <summary>
		/// Checks if date is business day
		/// </summary>
		public static bool IsBusinessDay(string date) => !IsWeekend(date);

		/// <summary>
		/// Adds business days to date
		/// </summary>
		public static string AddBusinessDays(string date, int days)
		{
			var dt = DateTime.Parse(date);
			var added = 0;
			while (added < days)
			{
				dt = dt.AddDays(1);
				if (IsBusinessDay(dt.ToString("yyyy-MM-dd")))
				{
					added++;
				}
			}
			return dt.ToString("yyyy-MM-dd");
		}

		/// <summary>
		/// Gets days between two dates
		/// </summary>
		public static int GetDaysBetween(string date1, string date2)
		{
			var dt1 = DateTime.Parse(date1);
			var dt2 = DateTime.Parse(date2);
			return (int)(dt2 - dt1).TotalDays;
		}

		/// <summary>
		/// Calculates age from birth date
		/// </summary>
		public static int GetAge(string birthDate)
		{
			var birth = DateTime.Parse(birthDate);
			var today = DateTime.Today;
			var age = today.Year - birth.Year;
			if (birth.Date > today.AddYears(-age)) age--;
			return age;
		}

		/// <summary>
		/// Checks if date is leap day
		/// </summary>
		public static bool IsLeapDay(string date)
		{
			var dt = DateTime.Parse(date);
			return dt.Month == 2 && dt.Day == 29;
		}

		/// <summary>
		/// Adds days to date
		/// </summary>
		public static string AddDays(string date, int n) => DateTime.Parse(date).AddDays(n).ToString("yyyy-MM-dd");

		/// <summary>
		/// Adds months to date
		/// </summary>
		public static string AddMonths(string date, int n) => DateTime.Parse(date).AddMonths(n).ToString("yyyy-MM-dd");

		/// <summary>
		/// Adds years to date
		/// </summary>
		public static string AddYears(string date, int n) => DateTime.Parse(date).AddYears(n).ToString("yyyy-MM-dd");

		/// <summary>
		/// Parses date string
		/// </summary>
		public static string ParseDate(string str) => DateTime.Parse(str).ToString("yyyy-MM-dd");
	}
}
