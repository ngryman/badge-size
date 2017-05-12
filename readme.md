# badge-size [![npm][npm-image]][npm-url] [![travis][travis-image]][travis-url]

[npm-image]: https://img.shields.io/npm/v/badge-size.svg?style=flat
[npm-url]: https://npmjs.org/package/badge-size
[travis-image]: https://img.shields.io/travis/ngryman/badge-size.svg?style=flat
[travis-url]: https://travis-ci.org/ngryman/badge-size

> Displays the size of a given file in your repository.


`badge-size` allows you to display in real time the size of a given file which lives in your repository.
The size is always the one of your last pushed commit.

It is mainly designed for front-end library authors that want to advertise the weight of
their builds to their users. But you can use it for any other purpose of course :v:.


## Examples

 Badge       | URL
:------------|:---------------------------------------------------------------------------------|
Normal size  | ![](http://img.badgesize.io/ngryman/badge-size/master/index.js.svg)
Gzipped size | ![](http://img.badgesize.io/ngryman/badge-size/master/index.js.svg?compression=gzip)
Custom label | ![](http://img.badgesize.io/ngryman/badge-size/master/index.js.svg?label=As_tiny_as)
PNG format   | ![](http://img.badgesize.io/ngryman/badge-size/master/index.js.png)
JPG format   | ![](http://img.badgesize.io/ngryman/badge-size/master/index.js.jpg)


## Usage

It works like any other badge service you may know and it's configurable in the image url itself.
Here is the general pattern of a typical `badge-size` url:

```
http://img.badgesize.io/:filepath[.svg|png|jpg][?compression=gzip][&label=string]
```

#### `:filepath`

Relative URL of file on GitHub of any absolute URL if hosted elsewhere.

The format of the GitHub URL is the same as when you browse it in the source explorer, minus `blob/` part.
Here is its typical form:

```md
:user/:repo/:branch/:path
```

For example if I want to point to this repository `index.js`, it would be:

`https://github.com/`**`ngryman/badge-size/master/index.js`**

Note that the branch name is mandatory.

#### `[.svg|png|jpg]`

Optional image format. By default `svg` is used.

#### `[?compression=gzip]`

Optional compression format to measure. It's useful if you want to advertise the *true* size your
file would take on the wire, assuming the server has `gzip` compression enabled.

#### `[&label=string]`

Optional text to display in the badge instead of *size* / *gzip size*.

#### `[&color=string]`

Optional background color. By default it's `brightgreen`.<br>
You can specify hexadecimal colors, without the dash (i.e `bada55`) or one of the following named
colors:

![](https://img.shields.io/badge/color-brightgreen-brightgreen.svg)
![](https://img.shields.io/badge/color-green-green.svg)
![](https://img.shields.io/badge/color-yellowgreen-yellowgreen.svg)
![](https://img.shields.io/badge/color-yellow-yellow.svg)
![](https://img.shields.io/badge/color-orange-orange.svg)
![](https://img.shields.io/badge/color-red-red.svg)
![](https://img.shields.io/badge/color-lightgrey-lightgrey.svg)
![](https://img.shields.io/badge/color-blue-blue.svg)

#### `[&style=string]`

Optional style. By default it's `flat`.<br>
You can specify one of the following:

![](https://img.shields.io/badge/style-plastic-brightgreen.svg?style=plastic)
![](https://img.shields.io/badge/style-flat-brightgreen.svg?style=flat)
![](https://img.shields.io/badge/style-flat--square-brightgreen.svg?style=flat-square)
![](https://img.shields.io/badge/style-social-brightgreen.svg?style=social)


## Contributors

[//]: contributor-faces
<a href="https://github.com/ngryman"><img src="https://avatars.githubusercontent.com/u/892048?v=3" title="ngryman" width="80" height="80"></a>
<a href="https://github.com/bfred-it"><img src="https://avatars.githubusercontent.com/u/1402241?v=3" title="bfred-it" width="80" height="80"></a>
<a href="https://github.com/nathancahill"><img src="https://avatars.githubusercontent.com/u/1383872?v=3" title="nathancahill" width="80" height="80"></a>
<a href="https://github.com/coopy"><img src="https://avatars.githubusercontent.com/u/794843?v=3" title="coopy" width="80" height="80"></a>
[//]: contributor-faces

<sup>Generated with [contributors-faces](https://github.com/ngryman/contributor-faces).</sup>


## License

MIT © [Nicolas Gryman](http://ngryman.sh)
