#include "../rust/libnewsboat-ffi/include/tuiwindow.h"

#include <iostream>

namespace newsboat {
namespace tui {
namespace bridged {

void draw_tui_window()
{
	std::cout << "test from draw_tui_window()" << std::endl;
}

}}}
