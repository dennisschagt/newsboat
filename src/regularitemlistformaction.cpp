#include "regularitemlistformaction.h"

#include "config.h"
#include "logger.h"
#include "rssfeed.h"
#include "strprintf.h"

namespace newsboat {

RegularItemListFormAction::RegularItemListFormAction(
	View* vv, std::string formstr, Cache* cc, FilterContainer& f,
	ConfigContainer* cfg, RegexManager& r)
	: ItemListFormAction(vv, formstr, cc, f, cfg, r)
{
}

std::string RegularItemListFormAction::get_title_format()
{
	return cfg->get_configvalue("articlelist-title-format");
}

std::string RegularItemListFormAction::title()
{
	if (feed->is_query_feed()) {
		return strprintf::fmt(_("Query Feed - %s"),
				feed->rssurl().substr(
					6, feed->rssurl().length() - 6));
	} else {
		auto feedtitle = feed->title();
		utils::remove_soft_hyphens(feedtitle);
		return strprintf::fmt(
				_("Article List - %s"), feedtitle);
	}
}

bool RegularItemListFormAction::process_operation(Operation op,
	bool automatic,
	std::vector<std::string>* args)
{
	const unsigned int itempos = list.get_position();

	switch (op) {
	case OP_OPEN:
		return open_item(itempos);
	case OP_RELOAD:
		LOG(Level::INFO, "ItemListFormAction: reloading current feed");
		v->get_ctrl()->get_reloader()->reload(pos);
		invalidate_list();
		return true;
	default:
		return ItemListFormAction::process_operation(op, automatic, args);
	}
}

} // namespace newsboat
