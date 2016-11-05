void board_init(void);
void console_putchar(char ch);
void console_puts(const char *s);
int  console_printf(const char *fmt, ...);
void print_reg(const char *label, uint32_t val);
void reset(void);
