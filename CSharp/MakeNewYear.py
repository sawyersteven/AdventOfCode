import os

abspath = os.path.abspath(__file__)
os.chdir(os.path.dirname(abspath))


template = """using AdventOfCode;

namespace AdventYYYY
{
    public class ChallengeDD : Challenge
    {
        public override object Task1()
        {
            return null;
        }

        public override object Task2()
        {
            return null;
        }
    }
}
"""


year = input("Year [eg 2019]:")

template = template.replace("YYYY", year)

if not os.path.exists(f"./{year}"):
    print(f"Making directory ./{year}")
    os.makedirs(f"./{year}")

if not os.path.exists(f"../Input/{year}"):
    print(f"Making directory ../Input/{year}")
    os.makedirs(f"../Input/{year}")


for day in range(1, 26):
    d = f"{day:02d}"
    cs = f"./{year}/{d}.cs"
    if not os.path.isfile(cs):
        with open(cs, "w") as f:
            f.write(template.replace("DD", d))

for day in range(1, 26):
    inp = f"../Input/{year}/{day:02d}.txt"
    if not os.path.isfile(inp):
        open(inp, "w").close()
