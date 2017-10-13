#ifndef NEWSBOAT_LIST_FORMACTION_H_
#define NEWSBOAT_LIST_FORMACTION_H_

#include "formaction.h"

namespace newsboat {

class list_formaction : public formaction {
	public:
		list_formaction(view *, std::string formstr);
	protected:
		void open_unread_items_in_browser(std::shared_ptr<rss_feed> feed , bool markread);

};


}

#endif /* NEWSBOAT_LIST_FORMACTION_H_ */
