#include "searchitemlistformaction.h"

namespace newsboat {

SearchItemListFormAction::SearchItemListFormAction(View* vv,
	std::string formstr, Cache* cc, FilterContainer& f, ConfigContainer* cfg,
	RegexManager& r)
	: ItemListFormAction(vv, formstr, cc, f, cfg, r)
{
	show_searchresult = true;
}

} // namespace newsboat
