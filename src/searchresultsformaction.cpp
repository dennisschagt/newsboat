#include "searchresultsformaction.h"

namespace newsboat {

SearchResultsFormAction::SearchResultsFormAction(
	View* vv, std::string formstr, Cache* cc, FilterContainer& f,
	ConfigContainer* cfg, RegexManager& r)
	: ItemListFormAction(vv, formstr, cc, f, cfg, r)
{
}

} // namespace newsboat
