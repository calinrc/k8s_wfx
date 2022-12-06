/*
 *    Utilities.h file written and maintained by Calin Cocan
 *    Created on: May 18, 2015
 *
 * This work is free: you can redistribute it and/or modify it under the terms of Apache License Version 2.0
 *
 * This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.
 * See the License for more details.
 * You should have received a copy of the License along with this program. If not, see <http://choosealicense.com/licenses/apache-2.0/>.

 ********************************************************************************************************************* */

#ifndef INCLUDE_UTILITIES_H_
#define INCLUDE_UTILITIES_H_

#include <stddef.h>
#include <stdlib.h>
#ifdef LINUX
#include <pwd.h>
#include <unistd.h>
#else
#include <Shlobj.h>
#endif
#include "gendef.h"
#include <string.h>

class Utilities
{
public:
    static char *getUserHomeDir()
    {
        char *homedir = new char[MAX_PATH];
#ifdef LINUX
        const char *homeDirVar = NULL;
        if ((homeDirVar = getenv("HOME")) == NULL)
        {
            homeDirVar = getpwuid(getuid())->pw_dir;
        }
        if (homeDirVar != NULL)
        {
            strcpy(homedir, homeDirVar);
            return homedir;
        }
        else
        {
            delete[] homedir;
            return NULL;
        }

#else
        if (SHGetFolderPathA(NULL, CSIDL_PROFILE, NULL, 0, homedir) == S_OK)
        {
            return homedir;
        }
        else if (SHGetFolderPathA(NULL, CSIDL_LOCAL_APPDATA, NULL, 0, homedir) == S_OK)
        {
            return homedir;
        }
        else
        {
            delete[] homedir;
            return NULL;
        }
#endif
    }

    static void mkDirectory(const char *path)
    {
        struct stat st = {0};

        if (stat(path, &st) == -1)
        {
#ifdef LINUX
            mkdir(path, 0700);
#else
            CreateDirectory(path, NULL);
#endif
        }
    }

    static char *getPluginsDir(char *retPath, size_t *size)
    {
        return getAbsolutePath(PLUGINS_LOCATION, retPath, size);
    }

    static char *getPluginDir(char *retPath, size_t *size)
    {
        return getAbsolutePath(PLUGIN_LOCATION, retPath, size);
    }

    static char *getLogDir(char *retPath, size_t *size)
    {
        return getAbsolutePath(LOG_PATH, retPath, size);
    }

    static char *getLogFilePath(char *retPath, size_t *size)
    {
        return getAbsolutePath(FULL_LOG_PATH, retPath, size);
    }

private:
    Utilities();
    virtual ~Utilities();

    static char *getAbsolutePath(const char *relativePath, char *retPath, size_t *size)
    {
        char path[MAX_PATH];
        char *userHomeDir = getUserHomeDir();
        if (userHomeDir != NULL)
        {
            sprintf(path, "%s/%s", userHomeDir, relativePath);
            delete[] userHomeDir;
            size_t length = strlen(path);

            if (length < *size - 1)
            {
                *size = length;
                strcpy(retPath, path);
                return retPath;
            }
            else
            {
                *size = length;
                return NULL;
            }
        }
        else
        {
            *size = 0;
            return NULL;
        }
    }
};

#endif /* INCLUDE_UTILITIES_H_ */
