import React from 'react';
import { Link, Redirect } from 'react-router-dom';
import marked from 'marked';

const API_URL = process.env.FINDWORK_API;

export class Tabs extends React.Component {
    constructor(props) {
        super(props);
        this.state = { tabs: null };
    }

    componentDidMount() {
        fetch(API_URL).then(function(response) {
            return response.json();
        }).then(data => {
            this.setState({ tabs: data.tabs });
        });
    }

    render() {
        if (!this.state.tabs) {
            return <div>loading...</div>;
        }

        const urlTab = this.props.match.params.tab;
        const urlCategory = this.props.match.params.category;
        const tabs = this.state.tabs;
        let tab;
        if (!urlTab) {
            tab = tabs[0];
        } else {
            for (const t of tabs) {
                if (t.id === urlTab) {
                    tab = t;
                    break;
                }
            }
            if (!tab) {
                console.log('Could not match tab ' + urlTab);
                tab = tabs[0];
            }
        }
        return <div>
            <TabStrip tabs={tabs} current={tab.id} />
            <Tab tab={tab} category={urlCategory} />
        </div>;
    }
}

const TabStrip = (props) => {
    const current = props.current;
    let tabs = [];
    for (const t of props.tabs) {
        let className = "tab";
        if (t.id === current) {
            className = "activeTab";
        }
        tabs.push(<span id={t.id} className={className} key={t.id}><Link to={'/' + t.id}>{t.title}</Link></span>);
    }
    return <div className="tabStrip">
        {tabs}
    </div>;
};

const Tab = (props) => {
    let body;
    let showAll = false;
    if (!props.category) {
        body = <TabCategories categories={props.tab.categories} tab={props.tab.id} />;
    } else if (props.category === 'all') {
        showAll = true;
        body = <TabAllIssues categories={props.tab.categories} tab={props.tab} />;
    } else {
        let category;
        for (const cat of props.tab.categories) {
            if (cat.id === props.category) {
                category = cat;
                break
            }
        }
        if (category) {
            body = <TabCategory category={category} tab={props.tab} />;
        } else {
            console.log('Unknown category ' + props.category + ' for tab ' + props.tab.id);
            body = <Redirect to={'/' + props.tab.id} />;
        }
    }
    const desc = marked(props.tab.description);
    return <div className="tabBody">
        <div className="tabHeader">
            <div className="tabHeaderText" dangerouslySetInnerHTML={{__html: desc}} />
            <TabOptions tags={props.tab.tags} tab={props.tab.id} showAll={showAll} />
        </div>
        {body}
    </div>;
};


const TabOptions = (props) => {
    let tags = [];
    // TODO make tags actually do something and then re-enable them.
    // for (const t of props.tags) {
    //     tags.push(<div className='tag' key={t}>{t}</div>);
    // }
    let showAll = null;
    if (!props.showAll) {
        showAll = <Link to={'/' + props.tab + '/all'}>show all issues</Link>;
    }
    return <div className="tabHeaderOptions">
            {showAll}
            <div className='tags'>{tags}</div>
    </div>;
};

const TabCategory = (props) => {
    // TODO tags
    let links = null;
    if (props.category.links.length) {
        let linkList = [];
        for (const l of props.category.links) {
            linkList.push(<li className="categoryLink" key={l.url}><a href={l.url} target="_blank">{l.text}</a></li>);
        }
        links = <ul className="categoryLinks">{linkList}</ul>;
    }
    const desc = marked(props.category.description);
    return <div className="tabCategory">
        <div className="back"><Link to={'/' + props.tab.id}>&lt;&lt; back to {props.tab.title}</Link></div>
        <h3 className="categoryTitle">{props.category.title}</h3>
        <div className="categoryDesc" dangerouslySetInnerHTML={{__html: desc}} />
        {links}
        <IssueList issues={props.category.issues} />
    </div>;
};

const TabCategories = (props) => {
    let cats = [];
    for (const cat of props.categories) {
        const length = cat.issues.length;
        let issueCounter = length + ' issues';
        if (length === 1) {
          issueCounter = length + ' issue';
        }
        const desc = marked(cat.description);
        const link = '/' + props.tab + '/' + cat.id;
        cats.push(<div className="shortCategory" key={cat.id}>
                     <h3 className = "categoryTitle"><Link to={link}>{cat.title}</Link></h3>
                     <div className = "categoryDesc" dangerouslySetInnerHTML={{__html: desc}} />
                     <Link to={link}>{issueCounter}</Link>
                  </div>);
    }
    return <div className="tabCategories">
        {cats}
    </div>;
};

const TabAllIssues = (props) => {
    let issues = [];
    for (const cat of props.categories) {
        issues.push(...cat.issues);
    }
    return <div className="tabAllIssues">
        <div className="back"><Link to={'/' + props.tab.id}>&lt;&lt; back to {props.tab.title}</Link></div>
        <IssueList issues={issues} />
    </div>;
};

export class IssueList extends React.Component {
    constructor(props) {
        super(props);
        this.state = { expanded: props.issues.map(i => false) };
    }

    render() {
        let issues = [];
        for (const i in this.props.issues) {
            const issue = this.props.issues[i];

            let body = issue.body.trim();
            let bodyMore = null;
            const linebreak = body.indexOf('\n');
            if (!this.state.expanded[i] && linebreak > 0) {
                body = body.substring(0, linebreak);
                const showMore = () => {
                    this.setState(prevState => {
                        let expanded = prevState.expanded.slice();
                        expanded[i] = true;
                        return { expanded };
                    });
                };
                bodyMore = <span className="issueMore" onClick={showMore}>...</span>;
            }

            body = marked(body);

            let labels = [];
            for (const l of issue.labels) {
                labels.push(<span className='issueLabel' key={l.name}>{l.name}</span>);
            }
            issues.push(
                <div className='issue' key={i}>
                    <a href={issue.html_url} target="_blank">{issue.title}</a>
                    <div className="issueBody" dangerouslySetInnerHTML={{__html: body}} />
                    {bodyMore}
                    <div className="issueLabels">{labels}</div>
                </div>
            );
        }
        return <div className="issues">
            {issues}
        </div>;
    }
}
