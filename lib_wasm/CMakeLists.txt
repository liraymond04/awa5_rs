set(CMAKE_POSITION_INDEPENDENT_CODE ON)
include_directories("/usr/local/include")
link_directories("./")

set(CMAKE_SHARED_LIBRARY_CREATE_C_FLAGS "-s SIDE_MODULE=2")
set(BUILD_SHARED_LIBS OFF)

add_library(web_scene STATIC "lib/awa5_raylib.c")
target_link_libraries(web_scene PUBLIC raylib)
set_target_properties(web_scene PROPERTIES SUFFIX ".a")

install(TARGETS web_scene DESTINATION lib)
install(FILES "${CMAKE_CURRENT_SOURCE_DIR}/libraylib.a" DESTINATION ".")

install(TARGETS web_scene DESTINATION .)
