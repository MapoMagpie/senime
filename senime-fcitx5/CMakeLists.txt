cmake_minimum_required(VERSION 3.16)
project(senime-fcitx5 VERSION 0.0.1 LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)

find_package(Fcitx5Core REQUIRED)
find_package(Fcitx5Config REQUIRED)
find_package(Fcitx5Utils REQUIRED)
find_package(Gettext REQUIRED)

add_definitions(-DFCITX_GETTEXT_DOMAIN=\"senime\")
fcitx5_add_i18n_definition()
add_subdirectory(po)

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
fcitx5_translate_desktop_file(data/fcitx5/addon/senime.conf.in senime.conf)
install(FILES "${CMAKE_CURRENT_BINARY_DIR}/senime.conf"
        DESTINATION "${FCITX_INSTALL_PKGDATADIR}/addon")
fcitx5_translate_desktop_file(data/fcitx5/inputmethod/senime.conf.in senime-im.conf)
install(FILES "${CMAKE_CURRENT_BINARY_DIR}/senime-im.conf"
        DESTINATION "${FCITX_INSTALL_PKGDATADIR}/inputmethod"
        RENAME senime.conf)
install(FILES data/fcitx5/icon/fcitx-senime-cn.svg data/fcitx5/icon/fcitx-senime-en.svg
        DESTINATION "${CMAKE_INSTALL_PREFIX}/share/icons/hicolor/scalable/apps")
install(DIRECTORY data/16x16 data/22x22 data/24x24 data/48x48 data/64x64
        DESTINATION "${CMAKE_INSTALL_DATADIR}/icons/hicolor"
        PATTERN .* EXCLUDE
        PATTERN *~ EXCLUDE)
