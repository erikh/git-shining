# git-shining: Transform the GitHub contributor graph into living art

This program turns your contributor graph into a work of art by filing commits with the right dates. GitHub will automatically scan the repository and fill out the graph for you.

<p><center><img src="heart.png" /></center></p>
<p><center><img src="font.png" /></center></p>

## Constraints

Best to create or use a burner account for this; this is not for your main account.

-   Your account must be at least a year old
-   You must have no other activity for the year, which includes pull requests, issue comments, etc.
-   You must only have one repository recording commits for the year, and it must be all commits related to your message.

## Usage

The first step is to generate configuration. You can do this in two ways:

```
git-shining generate-config <json|txt>
```

This will generate a blank canvas for editing in JSON or TXT (more on this later) format.

**or**

```
git-shining render-font <.otf or .ttf file> <message (be sure to quote)>
```

This will generate a pre-filled configuration with the message rasterized to the canvas in the font provided.

Then, you can generate a repository with the `generate-repository` command. Be sure to at least set the `email` field of the person you want getting credit for the commits, it must map directly to a configured GitHub email:

```
git-shining generate-repository -i bro.txt -e wearelegion@example.org -m "Sup Bro" /tmp/test.git
```

After that, you can create your repository and upload it.

## Configuration Formats

There are two formats for configuration: JSON and TXT.

JSON is a JSON array of integers, from 0-10, which coordinate to shading in the graph (made possible by filing more commits for brighter shades). It is easy to consume, post-process, and generate, but a little hard to edit.

TXT is an easier format to hand-edit. It is simply a well-aligned grid of integers, which you can set from 0-`A` (`A` is 10 here) to affects shading.

The `generate-config` sub-command can generate both formats for hand-editing, and the `render-font` command has switches to modify target configuration output.

## Troubleshooting

-   If you upload twice, you must fully delete the old repository and re-create it. Force pushes will not clear the graph.
-   There is no functionality (yet) to set the origin date, it just works off the current date.
-   If you're wondering what your graph will look like before you push to GitHub, try the `build` sub-command which will generate a HTML mock of the graph you can load into your browser. You can hover over each square to get the expected date.

## Future Plans

-   Set origin date of commits
-   Marquee and Mural functionality

## Author

The Professional <erik+github@hollensbe.org>
