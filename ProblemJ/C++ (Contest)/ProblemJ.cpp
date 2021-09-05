#include <iostream>
using namespace std;

int main()
{

    int cases;
    cin >> cases;
    int testnumbers[cases];

    for (int x = 0; x < cases; x++)
    {
        cin >> testnumbers[x];
    }

    int total = 0;

    for (int x = 0; x < cases; x++)
    {
        total = 0;
        for (int y = 1; y < testnumbers[x]; y++)
        {
            if (testnumbers[x] % y == 0)
            {
                total = total + y;
                cout << y << " ";
            }
        }
        if (total == testnumbers[x])
        {
            cout << "YES\n";
        }
        else
        {
            cout << "NO\n";
        }
    }
}
