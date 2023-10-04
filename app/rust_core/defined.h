typedef struct ExposedStrings {
    const char* summaries;
    const char* descriptions;
    const char* start_dates;
    const char* end_dates;
    const char* uuid;
} ExposedStrings;

void hello_devworld();
void print_string_to_console(const char* input_string, const char* input_string2, const char* input_string3);
void helloworld();
ExposedStrings expose_strings(void);
void deleteBlockCall(const char* input);
void create_file();
