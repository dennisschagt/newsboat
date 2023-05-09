#pragma once

#include <string>

namespace newsboat {
namespace tui {
namespace bridged {

void draw_tui_window();

class IWindow {
public:
	virtual ~IWindow() = default;

	virtual const std::string& get_title() = 0;
};

}}}
