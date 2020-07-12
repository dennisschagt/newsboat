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

private:
	bool process_operation(Operation op,
		bool automatic = false,
		std::vector<std::string>* args = nullptr) override;
};

} // namespace newsboat

#endif /* NEWSBOAT_REGULARITEMLISTFORMACTION_H_ */
