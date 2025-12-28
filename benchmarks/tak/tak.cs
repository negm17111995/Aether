// Tak Benchmark - C#
// Build: dotnet build -c Release
// Run: dotnet run -c Release

class Program {
    static int Tak(int x, int y, int z) {
        if (y >= x) return z;
        return Tak(Tak(x - 1, y, z), Tak(y - 1, z, x), Tak(z - 1, x, y));
    }

    static void Main() {
        int result = Tak(24, 16, 8);
        Console.WriteLine(result);
    }
}
