cmake_minimum_required(VERSION 3.0)
project(grayscale_host C)

set(WAMR_ROOT_DIR ${CMAKE_CURRENT_SOURCE_DIR}/../../wasm-micro-runtime)
set(WAMR_BUILD_DIR ${WAMR_ROOT_DIR}/samples/basic/build)

include_directories(
   ${WAMR_ROOT_DIR}/core/iwasm/include
   ${WAMR_ROOT_DIR}/core/shared/utils
   ${WAMR_ROOT_DIR}/core/shared/platform/linux
)

link_directories(${WAMR_BUILD_DIR})

add_executable(grayscale_host main.c)
target_link_libraries(grayscale_host vmlib -ldl -lm)