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

private:
	bool process_operation(Operation op,
		bool automatic = false,
		std::vector<std::string>* args = nullptr) override;

	std::string get_title_format() override;
	void show_article(std::string guid) override;
	void show_search_results(std::shared_ptr<RssFeed> results,
		std::string phrase) override;
};

} // namespace newsboat

#endif /* NEWSBOAT_REGULARITEMLISTFORMACTION_H_ */
