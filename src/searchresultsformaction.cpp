#include "searchresultsformaction.h"

#include "config.h"
#include "logger.h"
#include "strprintf.h"

namespace newsboat {

SearchResultsFormAction::SearchResultsFormAction(
	View* vv, std::string formstr, Cache* cc, FilterContainer& f,
	ConfigContainer* cfg, RegexManager& r, const std::string& s)
	: ItemListFormAction(vv, formstr, cc, f, cfg, r)
	, search_phrase(s)
{
}

std::string SearchResultsFormAction::get_title_format()
{
	return cfg->get_configvalue("searchresult-title-format");
}

std::string SearchResultsFormAction::title()
{
	return strprintf::fmt(_("Search Result - '%s'"), search_phrase);
}

bool SearchResultsFormAction::process_operation(Operation op,
	bool automatic,
	std::vector<std::string>* args)
{
	const unsigned int itempos = list.get_position();

	switch (op) {
	case OP_OPEN:
		return open_item(itempos, search_phrase);
	case OP_RELOAD:
		v->show_error( _("Error: you can't reload search results."));
		return false;
	default:
		return ItemListFormAction::process_operation(op, automatic, args);
	}
}

} // namespace newsboat
