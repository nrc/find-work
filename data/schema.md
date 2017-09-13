# Schema docs

The data in this directory defines the categories and header text used by the
site. In the code it is called 'structural' data. The actual issue data is
pulled directly from GitHub. These structural data are pulled directly from
https://github.com/nrc/find-work by the backend. To make changes, you merge a
change to that repo and wait for the next backend refresh.

`config.json` is somewhat different, it is taken from a local data directory and
is read only once at startup.

## [tabs](tabs.json)

A tab is the broadest category in the program. For example, 'starter issues' or
'projects'. A typical user will only want to browse one tab per visit.

```
[{
    "id": String,
    "title": String,
    "description": String
},
...]
```

* `id`: not exposed to the user, it is used for DOM ids and for relating backing
  data.
* `title`: a short title for the tab, rendered on the tab itself.
* `description`: markdown; rendered at the top of the tab.


## [categories](categories.json)

A category is a tool, library, or other class of work item. It appears in any
tab for which there is a `tab-category` (see below).

```
[{
    "id": String,
    "title": String,
    "description": String,
    "repository": String,
    "labels": [String],
    "links": [Link],
    "tags": [Tag]
},
...]
```

* `id`: not exposed to the user, it is used for DOM ids and for relating backing
  data.
* `title`: a short title for the tab.
* `description`: markdown; rendered for each category in each tab.
* `repository`: "user/name", e.g., "rust-lang-nursery/rustfmt", used to pull
  issue data, rendered as a link under the description.
* `labels`: issues must have all the given lables to be selected. Not shown to
  the user (though we do show labels for each issue).
* `links`: rendered under the description.
* `tags`: unimplemented!().


## [tab-category](tab-category.json)

Used to cross-reference tabs and categories (an n to n relation). Categories are
only rendered for tabs where an entry exists here.

```
[{
    "tab": String,
    "category": String,
    "labels": [String],
    "milestone": String | null,
    "link": String | null
},
...]
```

* `tab`: the `id` of a tab.
* `category`: the `id` of a category.
* `labels`: concatenated with the labels from the category, used to pull issues.
  Not shown to the user.
* `milestone`: used to pull issues; an issue must have the milestone if present
  and have all labels.
* `link`: a link rendered with category links under the description.


# `Link`

```
{
    "text": String,
    "url": String
}
```

* `text`: plain text description of the link, rendered inside the link's `a` tags
* `url`: the link's `href`, must be a valid URL.
