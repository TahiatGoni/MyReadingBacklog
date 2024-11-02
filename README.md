# My Reading Backlog
I spend a lot of time on sites like daily.dev to keep up with tech. I find myself opening so many tabs or bookmarking items that I never get to. So, I decided to create an application that can help me store links to all the articles I want to read. 

"Big deal! How is it any different from saving bookmarks?". 
I want to make this a terminal application that will show me my stats on startup. These stats will include how many articles I have added to my list, how many I have read already.
If it keeps popping up everytime I open a terminal, it will remind me to open up and read an article from time to time.
I want to be able to read an article in first in first out method or get a random article to read from my list.

MyReadingBacklog is a work in progress. It is intended to be a terminal application where you can store links to articles you want to read from the web.

### Update 2nd November 2024
- I have created the basic functionality
  - Decide to Use sqlite as a store for the articles.
  - This is a small project and I expect that anyone using it would not have many records.
  - I chose Rust because people talk about it and I think a small scale project like this is perfect to learn.
    - There have been some frustrations which I think are expected given my lack of experience with the language.
    - Shout out to ChatGPT for being super helpful.
      - What I am doing here is not super complex and there are defined ways to do things like writing to db, reading files etc.
      - I found it easier to ask chatGPT and this is the first time I have used it to this extent.
- Articles can be added
- Articles can returned in FIFO order
  - Using minimum Id for now to return articles.
  - A timestamp might be a better choice but minimum Id okay for now.
- Any returned Article id is saved so users can remove them when they are read.
- Statistics for articles read not implemented.
- Nothing implemented for setting up the program to show stats when the terminal is opened
- Some error Handling is missing with regards to passing arguments

### Core features
- Save links to articles into a queue.
- Get articles from the queue in FIFO order.
- Get random articles from the queue.
- Get stats on how many articles you have read and how many you have added to your list.
- On opening a terminal, show these stats to the user.

### Potential Additional features
- Add a category read feature to organize my articles and read from certain categories.
- Might be clumsy to manually add articles through the terminal, so research some UX improvements.

### Usage
The following arguments are available now

Add an article

```--add <link>```

Get article in FIFO order

```--get_queue_article```

Get a random article

```--get_random_article```

Mark last retrieved article as read. This removes the article from storage

```--article_read```
