#include "searchitemlistformaction.h"

#include "config.h"
#include "strprintf.h"

namespace newsboat {

SearchItemListFormAction::SearchItemListFormAction(View* vv,
	std::string formstr, Cache* cc, FilterContainer& f, ConfigContainer* cfg,
	RegexManager& r)
	: ItemListFormAction(vv, formstr, cc, f, cfg, r)
{
	show_searchresult = true;
}

bool SearchItemListFormAction::process_operation(Operation op,
	bool automatic,
	std::vector<std::string>* args)
{
	switch (op) {
	case OP_RELOAD:
		v->show_error(_("Error: you can't reload search results."));
		break;
	default:
		ItemListFormAction::process_operation(op, automatic, args);
		break;
	}
	return true;
}

std::string SearchItemListFormAction::get_title_format()
{
	return cfg->get_configvalue("searchresult-title-format");
}

std::string SearchItemListFormAction::title()
{
	return strprintf::fmt(_("Search Result - '%s'"), search_phrase);
}

} // namespace newsboat
