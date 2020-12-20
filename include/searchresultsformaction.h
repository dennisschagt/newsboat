#ifndef NEWSBOAT_SEARCHRESULTSFORMACTION_H_
#define NEWSBOAT_SEARCHRESULTSFORMACTION_H_

#include "itemlistformaction.h"

namespace newsboat {

class SearchResultsFormAction : public ItemListFormAction {
public:
	SearchResultsFormAction(View*,
		std::string formstr,
		Cache* cc,
		FilterContainer& f,
		ConfigContainer* cfg,
		RegexManager& r);
};

} // namespace newsboat

#endif /* NEWSBOAT_SEARCHRESULTSFORMACTION_H_ */
