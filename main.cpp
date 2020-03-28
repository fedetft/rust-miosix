
#include <cstdio>
#include "miosix.h"

using namespace std;
using namespace miosix;

extern "C" unsigned int duplicate(unsigned int);

int main()
{
    int result=duplicate(42);
    iprintf("%d\n",result);
}
