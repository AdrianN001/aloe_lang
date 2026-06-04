version_name="avocado"
aloe_language_lib_dir="$HOME/.aloe"
std_lib_location_gh="https://github.com/AdrianN001/aloe-std-lib"

create_aloe_lang_dir(){
    echo "$aloe_language_lib_dir/std"
    mkdir -p "$aloe_language_lib_dir/std"
}

create_dir_for_current_version(){
    echo "$aloe_language_lib_dir/std/$version_name"  
    mkdir -p "$aloe_language_lib_dir/std/$version_name"
}

fetch_standard_library_to_std_lib_location(){
    rm -rf "$aloe_language_lib_dir/std/$version_name"
    git clone "$std_lib_location_gh" \
        "$aloe_language_lib_dir/std/$version_name"
}

create_aloe_lang_dir
create_dir_for_current_version
fetch_standard_library_to_std_lib_location