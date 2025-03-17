/*
 *    k8s_wfx_test.cpp file written and maintained by Calin Cocan
 *    Created on: Dec 06, 2022
 *
 * This work is free: you can redistribute it and/or modify it under the terms of Apache License Version 2.0
 *
 * This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.
 * See the License for more details.
 * You should have received a copy of the License along with this program. If not, see <http://choosealicense.com/licenses/apache-2.0/>.

 ********************************************************************************************************************* */

#define LINUX

#include <iostream>
#include "wfxplugin.h"
#include <string.h>
#include "gendef.h"
#include <stdio.h>
#include "Utilities.h"

using namespace std;

typedef void (*FsGetDefRootName_func)(char *DefRootName, int maxlen);
typedef int (*FsInit_func)(int PluginNr, tProgressProc pProgressProc, tLogProc pLogProc, tRequestProc pRequestProc);
typedef void (*FsSetDefaultParams_func)(FsDefaultParamStruct *dps);
typedef HANDLE (*FsFindFirst_func)(char *Path, WIN32_FIND_DATAA *FindData);
typedef BOOL (*FsFindNext_func)(HANDLE Hdl, WIN32_FIND_DATAA *FindData);
typedef int (*FsFindClose_func)(HANDLE Hdl);

HANDLE INVALID_HANDLE = (HANDLE)-1;

#ifdef LINUX
#define LIB_REL_PATH "./target/debug/libk8s_wfx.dylib"
#else
#define LIB_REL_PATH "..\\target\\debug\\k8s.wfx"
#endif

void printFileInfo(char *path, WIN32_FIND_DATAA *data)
{
    printf("Informations about path %s:\n", path);
    printf("\t FileName: %s\n\t dwFileAttributes: %d\n\t dwReserved0: %d\n", data->cFileName, data->dwFileAttributes, data->dwReserved0);
}

int main()
{
    cout << "Begin" << endl; // prints !!!Hello World!!!
    char name[100];
    char lib_path[MAX_PATH];
    memset(lib_path, 0, sizeof(lib_path));
    char *userHomeDir = Utilities::getUserHomeDir();
    // snprintf(lib_path, MAX_PATH, "%s", LIB_REL_PATH);
    //snprintf(lib_path, MAX_PATH, "%s%s", userHomeDir, LIB_REL_PATH);
    strcpy(lib_path, LIB_REL_PATH);
    delete[] userHomeDir;

    LIB_HANDLER handle = LOAD_LIB(lib_path, RTLD_NOW);

    FsGetDefRootName_func FsGetDefRootName = (FsGetDefRootName_func)LOAD_PROC(handle, "FsGetDefRootName");
    FsInit_func FsInit = (FsInit_func)LOAD_PROC(handle, "FsInit");
    FsSetDefaultParams_func FsSetDefaultParams = (FsSetDefaultParams_func)LOAD_PROC(handle, "FsSetDefaultParams");
    FsFindFirst_func FsFindFirst = (FsFindFirst_func)LOAD_PROC(handle, "FsFindFirst");
    FsFindNext_func FsFindNext = (FsFindNext_func)LOAD_PROC(handle, "FsFindNext");
    FsFindClose_func FsFindClose = (FsFindClose_func)LOAD_PROC(handle, "FsFindClose");

    FsGetDefRootName(name, 100);

    cout << "Plugin Name: " << name << endl;

    FsInit(1, NULL, NULL, NULL);
    cout << "After FSInit";

    FsDefaultParamStruct dps;
    strcpy(dps.DefaultIniName, "Name1");
    dps.PluginInterfaceVersionHi = 3;
    dps.PluginInterfaceVersionLow = 2;
    dps.size = 5;

    FsSetDefaultParams(&dps);

    WIN32_FIND_DATAA FindData;
    memset(&FindData, 0, sizeof(WIN32_FIND_DATAA));

    char path[256] = {'/', '\0'};

    HANDLE h = FsFindFirst(path, &FindData);
    BOOL hasNext = true;
    if (h != NULL && h != INVALID_HANDLE)
    {
        while (hasNext)
        {
            printFileInfo(path, &FindData);
            hasNext = FsFindNext(h, &FindData);
        }
    }

    int closeResult = FsFindClose(h);
    printf("FsFindClose result %d\n", closeResult);

    strcpy(path, "/pod");

    h = FsFindFirst(path, &FindData);
    hasNext = true;
    if (h != NULL && h != INVALID_HANDLE)
    {
        while (hasNext)
        {
            printFileInfo(path, &FindData);
            hasNext = FsFindNext(h, &FindData);
        }
    }

    closeResult = FsFindClose(h);
    printf("FsFindClose result %d\n", closeResult);    

    return 0;
}
