#include "regularitemlistformaction.h"

#include "logger.h"
#include "rssfeed.h"
#include "strprintf.h"

namespace newsboat {

RegularItemListFormAction::RegularItemListFormAction(View* vv,
	std::string formstr, Cache* cc, FilterContainer& f, ConfigContainer* cfg,
	RegexManager& r)
	: ItemListFormAction(vv, formstr, cc, f, cfg, r)
{
}

bool RegularItemListFormAction::process_operation(Operation op,
	bool automatic,
	std::vector<std::string>* args)
{
	switch (op) {
	case OP_RELOAD:
		LOG(Level::INFO, "ItemListFormAction: reloading current feed");
		v->get_ctrl()->get_reloader()->reload(pos);
		invalidate_everything();
		break;
	default:
		ItemListFormAction::process_operation(op, automatic, args);
		break;
	}
	return true;
}

std::string RegularItemListFormAction::get_title_format()
{
	return cfg->get_configvalue("articlelist-title-format");
}

std::string RegularItemListFormAction::title()
{
	if (feed->is_query_feed()) {
		return strprintf::fmt(
				_("Query Feed - %s"),
				feed->rssurl().substr(6));
	} else {
		auto feedtitle = feed->title();
		utils::remove_soft_hyphens(feedtitle);
		return strprintf::fmt(
				_("Article List - %s"), feedtitle);
	}
}

void RegularItemListFormAction::show_article(std::string guid)
{
	v->push_itemview(feed, guid);
}

void RegularItemListFormAction::show_search_results(std::shared_ptr<RssFeed>
	results, std::string phrase)
{
	v->push_searchresult(results, phrase);
}

} // namespace newsboat
