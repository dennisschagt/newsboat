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
		RegexManager& r,
		const std::string& s);

	std::string id() const override
	{
		return "articlelist";
	}

	std::string title() override;

	bool process_operation(Operation op, bool automatic,
		std::vector<std::string>* args) override;

	bool this_is_search_result() override
	{
		return true;
	}

private:
	std::string get_title_format() override;

	const std::string search_phrase;
};

} // namespace newsboat

#endif /* NEWSBOAT_SEARCHRESULTSFORMACTION_H_ */
