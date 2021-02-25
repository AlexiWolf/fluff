# Fluff

Fluff helps you track your stuff!

Fluff is a small utility for rapid, spontaneous note-taking right from your command line.  It is useful for programmers,
sysadmins, or anyone who finds themself stuck in terminal-land on a regular basis.  Fluff is designed to be fast, easy
and powerful to use.  It supports simple note-taking, task and event tracking, and presentation with filtering (see the
usage documentation for more information.)

**Note:** Fluff is not yet complete.  Expect missing / incomplete features, bugs, and major changes until 1.0.

# Installation

TODO

# Usage

## Taking Notes

Fluff's default action is note-taking, so taking notes with Fluff is quick and easy.

```commandline
fluff Hello, world! ~ <3
```

Or with the `note` command.

```commandline
fluff note Calling note is unessesary, but you can do it anyways!
```

### Specialized Notes

#### Titled Notes

Notes don't require notes, but you can specify them if you like to keep things labeled.

```commandline
fluff I am a titled note! --title Titled Note
```

#### Tasks

Basic notes are useful for keeping little bits of information, but sometimes you'll want something more suited for 
tracking what you need to get done.  That's where tasks, and events come in!

To create a task, just tell fluff you're creating a task.

```commandline
fluff --task Email Jamie
```

Tasks may be assigned a due-date.

```commandline
fluff --task Fix all the bugs --date Jan 22 2021
```

If you prefer, you can also use `--due`, `--on`, or `--by` in place of `--date`.

```commandline
fluff --task Fix all the bugs --due Jan 22 2021
```

```commandline
fluff --task Fix all the bugs --on Jan 22 2021
```

```commandline
fluff --task Fix all the bugs --by Jan 22 2021
```

#### Events

Creating an event is similar to creating a task.  Unlike tasks, events require a date / time.

```commandline
fluff --event Code Review --date 02/20/21 13:00:00
```

## Viewing Your Notes

You can view your saved notes with:

```commandline
fluff view
```

The view command will show you the top 10 entries by default, but you can view more by specifying a number of entries 
to show.

```commandline
fluff view 100
```

You can also use the `count` flag.

```commandline
fluff view --count 100
```

This will show you the latest 100 entries.

To view everything, you can use the `all` flag.

```commandline
fluff view --all
```

### Basic Content Filtering

Fluff allows you to search for notes using various filters.

Filtering by content.

```commandline
fluff view --contains that one thing
```

Filtering by title.

```commandline
fluff view --title The Best Idea
```

Filtering by collection.

```commandline
fluff view --collection Shopping List
```

Filters can also be mixed.

```commandline
fluff view --collection Work Stuff --Title Meeting
```

You can still specify how many entries you want to see:

```commandline
fluff view 100 --collection A Large Set of Notes
```

### Filtering by Type

Searches can be limited to specific types of notes. For example, if you only want tasks.

```commandline
fluff view --tasks
```

You can also request events.

```commandline
fluff view --events
```

Or just basic notes.

```commandline
fluff view --notes
```

As with all filters, these can be mixed.

```commandline
fluff view --tasks --events
```

Content searching only tasks.

```commandline
fluff view --tasks --contains do a thing
```

### Date / Time Filtering

Entries can be filtered by date and time.  Fluff will do its best to handle basic date / time formats.

Only show notes before a certain date.

```commandline
fluff view --before April 20 2020
```

Only show notes after a certain time.

```commandline
fluff view --after 12-20-2018 10:20 
```

Only show notes within a date range.

```commandline
fluff view --after June 5 1993 --before May 15 2000
```

## Exporting Notes

Fluff allows you to export your notes.

```commandline
fluff export
```

or

```commandline
fluff export --all
```

The `export` command also supports all the same arguments and filters as the `view` command.

So you can specify a number of entries to export.

```commandline
fluff export 100
```

Or you can export just specific entry types: 

```commandline
fluff view --tasks
```

```commandline
fluff view --events
```

```commandline
fluff view --notes
```

```commandline
fluff view --tasks --events
```

Or entries within a date range:

```commandline
fluff export --after June 5 1993 --before May 15 2000
```

### Export Formats

TODO

# License

Copyright (C) 2021 Alexi Wolf

Fluff is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

Fluff is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with Fluff.  If not, see <https://www.gnu.org/licenses/>.