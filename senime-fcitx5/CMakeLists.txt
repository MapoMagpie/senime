cmake_minimum_required(VERSION 3.16)
project(senime-fcitx5 VERSION 0.0.1 LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)

find_package(Fcitx5Core REQUIRED)
find_package(Fcitx5Config REQUIRED)
find_package(Fcitx5Utils REQUIRED)

find_program(CARGO_EXECUTABLE cargo REQUIRED)

set(SENIME_RUST_TARGET_DIR "${CMAKE_CURRENT_BINARY_DIR}/cargo-target")
set(SENIME_RUST_LIB
    "${SENIME_RUST_TARGET_DIR}/release/${CMAKE_STATIC_LIBRARY_PREFIX}senime_fcitx5${CMAKE_STATIC_LIBRARY_SUFFIX}")

add_custom_command(
    OUTPUT "${SENIME_RUST_LIB}"
    COMMAND "${CMAKE_COMMAND}" -E env CARGO_TARGET_DIR=${SENIME_RUST_TARGET_DIR}
            "${CARGO_EXECUTABLE}" build --release -p senime-fcitx5
    WORKING_DIRECTORY "${CMAKE_CURRENT_SOURCE_DIR}/.."
    DEPENDS
        "${CMAKE_CURRENT_SOURCE_DIR}/Cargo.toml"
        "${CMAKE_CURRENT_SOURCE_DIR}/src/lib.rs"
        "${CMAKE_CURRENT_SOURCE_DIR}/../senime-lib/src/lib.rs"
        "${CMAKE_CURRENT_SOURCE_DIR}/../senime-lib/src/dict.rs"
        "${CMAKE_CURRENT_SOURCE_DIR}/../senime-lib/src/input_analyzer.rs"
        "${CMAKE_CURRENT_SOURCE_DIR}/../senime-lib/src/lookup_code.rs"
        "${CMAKE_CURRENT_SOURCE_DIR}/../senime-lib/src/util.rs"
    VERBATIM)

add_custom_target(senime_fcitx5_rust DEPENDS "${SENIME_RUST_LIB}")

add_library(senime_fcitx5_static STATIC IMPORTED GLOBAL)
set_target_properties(senime_fcitx5_static PROPERTIES
    IMPORTED_LOCATION "${SENIME_RUST_LIB}")
add_dependencies(senime_fcitx5_static senime_fcitx5_rust)

add_library(senime MODULE
    src/engine.cpp)
add_dependencies(senime senime_fcitx5_static)
target_include_directories(senime PRIVATE
    "${CMAKE_CURRENT_SOURCE_DIR}/include")
target_link_libraries(senime PRIVATE
    Fcitx5::Core
    Fcitx5::Config
    Fcitx5::Utils
    senime_fcitx5_static
    pthread
    dl
    m)
set_target_properties(senime PROPERTIES
    PREFIX "lib"
    OUTPUT_NAME "senime")

install(TARGETS senime DESTINATION "${FCITX_INSTALL_LIBDIR}/fcitx5")
install(FILES data/fcitx5/addon/senime.conf DESTINATION "${FCITX_INSTALL_PKGDATADIR}/addon")
install(FILES data/fcitx5/inputmethod/senime.conf DESTINATION "${FCITX_INSTALL_PKGDATADIR}/inputmethod")
install(FILES data/fcitx5/icon/fcitx-senime-cn.svg data/icon/fcitx-senime-en.svg
        DESTINATION "${CMAKE_INSTALL_PREFIX}/share/icons/hicolor/scalable/apps")
