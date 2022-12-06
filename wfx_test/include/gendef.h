/*
 *    gendef.h file written and maintained by Calin Cocan
 *    Created on: May 17, 2015
 *
 * This work is free: you can redistribute it and/or modify it under the terms of Apache License Version 2.0
 *
 * This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.
 * See the License for more details.
 * You should have received a copy of the License along with this program. If not, see <http://choosealicense.com/licenses/apache-2.0/>.

 ********************************************************************************************************************* */

#ifndef INCLUDE_GENDEF_H_
#define INCLUDE_GENDEF_H_

#include <stddef.h>

#define PLUGINS_LOCATION ".config"

#define PLUGIN_LOCATION PLUGINS_LOCATION "/k8s_wfx"
#define LOG_PATH PLUGIN_LOCATION "/logs"
#define FULL_LOG_PATH PLUGIN_LOCATION "/logs/k8s_wfx.log"

#define PATH_SEPARATOR ":"
#define FILE_SEPARATOR "/"

#ifdef LINUX

#include <dlfcn.h>
#include <sys/stat.h>
#include <sys/types.h>
#include <dirent.h>
#define LIB_HANDLER void *
#define LOAD_LIB(__PATH__, __PARAMS__) dlopen(__PATH__, __PARAMS__)
#define LOAD_PROC dlsym
#define FREE_LIB dlclose

#else

#include <windows.h>
#define LIB_HANDLER HINSTANCE
#define LOAD_LIB(__PATH__, __PARAMS__) LoadLibrary(__PATH__)
#define LOAD_PROC GetProcAddress
#define FREE_LIB FreeLibrary

#endif

#define MAX_PATH 260

#endif /* INCLUDE_GENDEF_H_ */
