#ifndef NEWSBOAT_REGULARITEMLISTFORMACTION_H_
#define NEWSBOAT_REGULARITEMLISTFORMACTION_H_

#include "itemlistformaction.h"

namespace newsboat {

class RegularItemListFormAction : public ItemListFormAction {
public:
	RegularItemListFormAction(View*,
		std::string formstr,
		Cache* cc,
		FilterContainer& f,
		ConfigContainer* cfg,
		RegexManager& r);

	std::string id() const override
	{
		return "articlelist";
	}

	std::string title() override;

	bool process_operation(Operation op, bool automatic,
		std::vector<std::string>* args) override;

	bool this_is_search_result() override
	{
		return false;
	}

private:
	std::string get_title_format() override;
};

} // namespace newsboat

#endif /* NEWSBOAT_REGULARITEMLISTFORMACTION_H_ */
