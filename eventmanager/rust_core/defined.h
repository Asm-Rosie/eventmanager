typedef struct ExposedStrings {
    const char* summaries;
    const char* descriptions;
    const char* start_dates;
    const char* end_dates;
    const char* uuid;
} ExposedStrings;

void print_string_to_console(const char* input_string, const char* input_string2, const char* input_string3);
ExposedStrings expose_strings(void);
void delete_block_call(const char* input);
void create_file();

