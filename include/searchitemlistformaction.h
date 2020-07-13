#ifndef NEWSBOAT_SEARCHITEMLISTFORMACTION_H_
#define NEWSBOAT_SEARCHITEMLISTFORMACTION_H_

#include "itemlistformaction.h"

namespace newsboat {

class SearchItemListFormAction : public ItemListFormAction {
public:
	SearchItemListFormAction(View*,
		std::string formstr,
		Cache* cc,
		FilterContainer& f,
		ConfigContainer* cfg,
		RegexManager& r,
		const std::string& searchphrase);

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

	const std::string search_phrase;
};

} // namespace newsboat

#endif /* NEWSBOAT_SEARCHITEMLISTFORMACTION_H_ */
