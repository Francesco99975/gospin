use std::vec;

pub struct ProjectFile {
    pub filename: String,
    pub content: String,
}

pub struct ProjectDir {
    pub dirname: String,
    pub files: Option<Vec<ProjectFile>>,
    pub dirs: Option<Vec<ProjectDir>>,
}

pub fn generate_project_structure(project: &str, port: u32, import_str: &str) -> ProjectDir {
    ProjectDir {
        dirname: project.to_string(),
        files: Some(vec![
            air_toml(),
            dockerignore(),
            dotenvf(port),
            prodenv(port),
            gitignore(),
            dockercompose(project, port),
            dockerfile(project, port),
            readme(),
        ]),
        dirs: Some(vec![
            ProjectDir {
                dirname: format!("assets"),
                dirs: None,
                files: None,
            },
            ProjectDir {
                dirname: format!("static"),
                dirs: None,
                files: None,
            },
            ProjectDir {
                dirname: format!("tmp"),
                dirs: None,
                files: None,
            },
            client_dir(),
            cmd_dir(import_str),
            internal(import_str),
            views(import_str),
        ]),
    }
}

fn client_dir() -> ProjectDir {
    let style = ProjectFile {
        filename: format!("style.css"),
        content: r#"@tailwind base;
@tailwind components;
@tailwind utilities;

/* @font-face {
  font-family: "JosefineSans";
  src: url("../fonts/JosefinSansRegular-x3LYV.ttf");
  font-display: swap;
} */

@layer base {
  :root {
    --color-std: 216 201 155;
    --color-primary: 39 62 71;
    --color-accent: 174 246 199;
    --color-success: 86 229 46;
    --color-error: 138 18 18;
  }
}

@layer {
  #logout {
    @apply w-full !bg-red-500 p-2 tracking-widest !border-red-500 !border-2 shadow-md !font-bold;
  }
}

html {
  scroll-behavior: smooth;
  /* font-family: "JosefineSans", "Segoe UI", Tahoma, Geneva, Verdana, sans-serif; */
  font-family: "Segoe UI", Tahoma, Geneva, Verdana, sans-serif;
}"#
        .to_string(),
    };

    let counter = ProjectFile {
        content: r#"import { LitElement, html, css } from "lit";
import { property } from "lit/decorators.js";

export class UCounter extends LitElement {
  @property({ type: String })
  idd: string = "";

  @property({ type: Number })
  min: number = 0;

  @property({ type: Number })
  value: number = 0;

  static styles = css`
    div.flex {
      display: flex;
      align-items: center;
      justify-content: space-evenly;
      width: 100%;
    }

    button {
      border-width: 4px;
      width: 3rem;
      height: 3rem;
      --tw-border-opacity: 1;
      border-color: rgb(var(--color-primary) / var(--tw-border-opacity));
      border-style: solid;
      border-radius: 9999px;
      text-align: center;
      --tw-text-opacity: 1;
      color: rgb(var(--color-primary) / var(--tw-text-opacity));
      font-weight: 700;
      font-size: 1.875rem;
      line-height: 2.25rem;
    }
  `;

  private _increase() {
    this.value = +this.value + 1;
  }

  private _decrease() {
    if (+this.value > +this.min) {
      this.value = +this.value - 1;
    }
  }

  protected render() {
    return html`
      <div class="flex">
        <input
          id="${"qty" + this.idd}"
          min="${this.min.toString()}"
          value="${this.value.toString()}"
          type="hidden"
        />
        <button id="${"dec" + this.idd}" @click="${this._decrease}">-</button>
        <span class="text-xl md:text-3xl font-bold p-2 text-center"
          >${this.value}</span
        >
        <button id="${"inc" + this.idd}" @click="${this._increase}">+</button>
      </div>
    `;
  }
}
"#
        .to_string(),
        filename: "ucounter.ts".to_string(),
    };

    let css = ProjectDir {
        dirname: format!("css"),
        files: Some(vec![style]),
        dirs: None,
    };

    let components = ProjectDir {
        dirname: format!("components"),
        files: Some(vec![counter]),
        dirs: None,
    };

    let fonts = ProjectDir {
        dirname: format!("fonts"),
        dirs: None,
        files: None,
    };

    let indexts = ProjectFile {
        filename: format!("index.ts"),
        content: r#"import "./css/style.css";

import htmx from "htmx.org";

declare global {
  interface Window {
    htmx: typeof htmx;
  }
}

window.htmx = htmx;"#
            .to_string(),
    };

    let src = ProjectDir {
        dirname: format!("src"),
        files: Some(vec![indexts]),
        dirs: Some(vec![css, fonts, components]),
    };

    let package = ProjectFile {
        filename: format!("package.json"),
        content: r#"{
  "name": "client",
  "version": "1.0.0",
  "description": "",
  "main": "index.js",
  "scripts": {
    "dev": "vite",
    "build:watch": "vite build --watch",
    "build": "vite build",
    "lint": "eslint . --ext .ts",
    "lint:fix": "eslint . --ext .ts --fix",
    "prettier": "prettier --write src/"
  },
  "author": "",
  "license": "ISC",
  "dependencies": {
    "htmx.org": "^1.9.10",
    "lit": "^3.1.2"
  },
  "devDependencies": {
    "autoprefixer": "^10.4.16",
    "postcss": "^8.4.31",
    "tailwindcss": "^3.3.5",
    "typescript": "^5.3.2",
    "vite": "^5.0.3"
  }
}"#
        .to_string(),
    };

    let postcss = ProjectFile {
        filename: format!("postcss.config.cjs"),
        content: r#"module.exports = {
  plugins: {
    tailwindcss: {},
    autoprefixer: {},
  },
};"#
        .to_string(),
    };

    let tailwind = ProjectFile {
        filename: format!("tailwind.config.cjs"),
        content: r#"/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    "./src/**/*.js",
    "../views/*.html",
    "../views/*.templ",
    "../views/components/*.templ",
    "../views/icons/*.templ",
    "../views/layouts/*.templ",
  ],
  theme: {
    extend: {
      colors: {
        std: "rgb(var(--color-std) / <alpha-value>)",
        primary: "rgb(var(--color-primary) / <alpha-value>)",
        success: "rgb(var(--color-success) / <alpha-value>)",
        accent: "rgb(var(--color-accent) / <alpha-value>)",
        error: "rgb(var(--color-error) / <alpha-value>)",
        transparent: "transparent",
        current: "currentColor",
      },
    },
  },
  plugins: [],
};"#
        .to_string(),
    };

    let tsconfig = ProjectFile {
        filename: format!("tsconfig.json"),
        content: r#"{
  "compilerOptions": {
    "rootDir": "src",
    "outDir": "dist",
    "strict": true,
    "lib": ["ES6", "DOM"],
    "moduleResolution": "node",
    "module": "ES2015",
    "target": "ES6",
    "sourceMap": true,
    "esModuleInterop": true,
    "allowJs": true,
    "checkJs": false,
    "experimentalDecorators": true,
    "resolveJsonModule": true
  },
  "include": ["src/**/*.ts"]
}"#
        .to_string(),
    };

    let vite = ProjectFile {
        filename: format!("vite.config.cjs"),
        content: r#"// vite.config.js
/** @type {import('vite').UserConfig} */
export default {
  base: "/assets/dist",
  build: {
    outDir: "../static/dist",
    rollupOptions: {
      input: "src/index.ts",
      output: {
        entryFileNames: "index.js",
        assetFileNames: "[name].[ext]",
      },
    },
  },
  plugins: [],
};"#
        .to_string(),
    };

    ProjectDir {
        dirname: format!("client"),
        dirs: Some(vec![src]),
        files: Some(vec![package, postcss, tailwind, tsconfig, vite]),
    }
}

fn cmd_dir(import_str: &str) -> ProjectDir {
    let config_go = ProjectFile {
        filename: format!("config.go"),
        content: r#"package boot

import (
	"fmt"

	"github.com/joho/godotenv"
)

func LoadEnvVariables() error {
	err := godotenv.Load(".env")
	if err != nil {
		return fmt.Errorf("cannot load environment variables")
	}

	return err
}
"#
        .to_string(),
    };

    let boot = ProjectDir {
        dirname: format!("boot"),
        files: Some(vec![config_go]),
        dirs: None,
    };

    let main_go = ProjectFile {
        filename: format!("main.go"),
        content: format!(
            r#"package main
import (
    "context"
	"fmt"
	"os"
    "os/signal"
	"time"

	"{0}/cmd/boot"
    "{0}/internal/models"
)

func main() {{
	err := boot.LoadEnvVariables()
	if err != nil {{
		panic(err)
	}}

	port := os.Getenv("PORT")

	models.Setup(os.Getenv("DSN"))

	e := createRouter()

    go func() {{
		fmt.Printf("Running Server on port %s", port)
		e.Logger.Fatal(e.Start(":" + port))
	}}()

	quit := make(chan os.Signal, 1)
	signal.Notify(quit, os.Interrupt)
	<-quit
	ctx, cancel := context.WithTimeout(context.Background(), 10*time.Second)
	defer cancel()
	if err := e.Shutdown(ctx); err != nil {{
		e.Logger.Fatal(err)
	}}
}}"#,
            import_str
        ),
    };

    let router_go = ProjectFile {
        filename: format!("router.go"),
        content: format!(
            r#"package main

import (
	"bytes"
	"context"
	"net/http"
    "time"

	"{0}/internal/controllers"
	"{0}/internal/models"
	"{0}/views"

	"github.com/labstack/echo/v4"
	"github.com/labstack/echo/v4/middleware"
    "github.com/labstack/gommon/log"
)

func createRouter() *echo.Echo {{
	e := echo.New()
	e.Use(middleware.Logger())
    e.Use(middleware.Recover())
	e.Use(middleware.RemoveTrailingSlash())
	e.Logger.SetLevel(log.INFO)
    e.GET("/healthcheck", func(c echo.Context) error {{
		time.Sleep(5 * time.Second)
		return c.JSON(http.StatusOK, "OK")
	}})

	e.Static("/assets", "./static")

	e.GET("/", controllers.Index())

	e.HTTPErrorHandler = serverErrorHandler

	return e
}}

func serverErrorHandler(err error, c echo.Context) {{
	code := http.StatusInternalServerError
	if he, ok := err.(*echo.HTTPError); ok {{
		code = he.Code
	}}
	data := models.GetDefaultSite("Error")

	buf := bytes.NewBuffer(nil)
	if code < 500 {{
		_ = views.ClientError(data, err).Render(context.Background(), buf)

	}} else {{
		_ = views.ServerError(data, err).Render(context.Background(), buf)
	}}

	_ = c.Blob(200, "text/html; charset=utf-8", buf.Bytes())

}}
"#,
            import_str
        ),
    };

    let server = ProjectDir {
        dirname: format!("server"),
        files: Some(vec![main_go, router_go]),
        dirs: None,
    };

    ProjectDir {
        dirname: format!("cmd"),
        files: None,
        dirs: Some(vec![boot, server]),
    }
}

fn internal(import_str: &str) -> ProjectDir {
    let index_go = ProjectFile {
        filename: format!("index.go"),
        content: format!(
            r#"package controllers

import (
	"net/http"

	"{0}/internal/helpers"
	"{0}/internal/models"
	"{0}/views"
	"github.com/labstack/echo/v4"
)

func Index() echo.HandlerFunc {{
	return func(c echo.Context) error {{
		data := models.GetDefaultSite("Home")

		html, err := helpers.GeneratePage(views.Index(data))

		if err != nil {{
			return echo.NewHTTPError(http.StatusBadRequest, "Could not parse page home")
		}}

		return c.Blob(200, "text/html; charset=utf-8", html)
	}}
}}"#,
            import_str
        ),
    };

    let controllers = ProjectDir {
        dirname: format!("controllers"),
        files: Some(vec![index_go]),
        dirs: None,
    };

    let page = ProjectFile {
        filename: format!("page.go"),
        content: r#"package helpers

import (
	"bytes"
	"context"

	"github.com/a-h/templ"
)

func GeneratePage(page templ.Component) ([]byte, error) {
	buf := bytes.NewBuffer(nil)

	err := page.Render(context.Background(), buf)

	if err != nil {
		return []byte{}, err
	}

	return buf.Bytes(), nil
}
"#
        .to_string(),
    };

    let helpers = ProjectDir {
        dirname: format!("helpers"),
        files: Some(vec![page]),
        dirs: None,
    };

    let middlewares = ProjectDir {
        dirname: format!("middlewares"),
        files: None,
        dirs: None,
    };

    let db = ProjectFile {
        content: r#"package models

import (
	"log"
	"os"

	"github.com/jmoiron/sqlx"
	_ "github.com/lib/pq"
)

var db *sqlx.DB

func Setup(dsn string) {
	var err error
	db, err = sqlx.Connect("postgres", dsn)
	if err != nil {
        log.Fatalln(err)
  }

	err = db.Ping()
	if err != nil {
        log.Fatalln(err)
  }

	schema, err := os.ReadFile("sql/init.sql")
	if err != nil {
        log.Fatalln(err)
  }

	db.MustExec(string(schema))
}
"#
        .to_string(),
        filename: "db.go".to_string(),
    };

    let site = ProjectFile {
        filename: format!("site.go"),
        content: r#"package models

import "time"

type SEO struct {
	Description string
	Keywords    string
}
type Site struct {
	AppName  string
	Title    string
	Metatags SEO
	Year     int
}

func GetDefaultSite(title string) Site {
	return Site{
		AppName:  "GoApp",
		Title:    title,
		Metatags: SEO{Description: "App", Keywords: "tool"},
		Year:     time.Now().Year(),
	}
}
"#
        .to_string(),
    };

    let models = ProjectDir {
        dirname: format!("models"),
        files: Some(vec![site, db]),
        dirs: None,
    };

    ProjectDir {
        dirname: format!("internal"),
        dirs: Some(vec![controllers, helpers, middlewares, models]),
        files: None,
    }
}

fn views(import_str: &str) -> ProjectDir {
    let header = ProjectFile {
        filename: format!("header.templ"),
        content: r#"package components

templ Header() {
	<header class="flex justify-between items-center bg-std text-center text-primary rounded-b w-full h-24 p-4 sticky top-0 right-0 z-20">
		<div class="flex items-center p-2 w-[35%]">
			<img src="/assets/images/logo.webp" alt="App Logo" width="85px" height="85px"/>
			<h1 class="ml-8 text-3xl"><a href="/">Title</a></h1>
		</div>
		<nav class="md:w-auto">
			<!-- Burger menu icon for small screens -->
			<div id="burgerMenu" class="burger-menu md:hidden cursor-pointer">
				<div id="bar1" class="bar w-6 h-1 bg-primary my-1 rounded transition-transform transform rotate-0"></div>
				<div id="bar2" class="bar w-6 h-1 bg-primary my-1 rounded transition-transform transform rotate-0"></div>
				<div id="bar3" class="bar w-6 h-1 bg-primary my-1 rounded transition-transform transform rotate-0"></div>
			</div>
			<!-- Navigation links for larger screens -->
			<ul id="navLinks" class="nav-links md:flex flex-row space-x-4 hidden">
				<li><a href="/#page1" class="text-primary text-xl md:text-2xl">Page 1</a></li>
				<li><a href="/#page2" class="text-primary text-xl md:text-2xl">Page 2</a></li>
				<li><a href="/#page3" class="text-primary text-xl md:text-2xl">Page 3</a></li>
				<li><a href="/#page4" class="text-primary text-xl md:text-2xl">Page 4</a></li>
				<li><a href="/#page5" class="text-primary text-xl md:text-2xl">Page 5</a></li>
			</ul>
			<!-- Navigation links for mobile view -->
			<ul id="mobileNavLinks" class="nav-links-mobile md:hidden absolute top-24 left-0 w-full hidden z-30 transition-all ease-in">
				<li class="bg-std w-full px-4 py-2"><a href="/#page1" class="text-primary text-center  text-xl md:text-2xl">Page 1</a></li>
				<li class="bg-std w-full px-4 py-2"><a href="/#page2" class="text-primary text-center  text-xl md:text-2xl">Page 2</a></li>
				<li class="bg-std w-full px-4 py-2"><a href="/#page3" class="text-primary text-center  text-xl md:text-2xl">Page 3</a></li>
				<li class="bg-std w-full px-4 py-2"><a href="/#page4" class="text-primary text-center  text-xl md:text-2xl">Page 4</a></li>
				<li class="bg-std w-full px-4 py-2"><a href="/#page5" class="text-primary text-center  text-xl md:text-2xl">Page 5</a></li>
			</ul>
		</nav>
		<script>

            var burgerMenu = document.getElementById('burgerMenu');
            var navLinks = document.getElementById('mobileNavLinks');
            var bar1 = document.getElementById('bar1');
            var bar2 = document.getElementById('bar2');
            var bar3 = document.getElementById('bar3');

            burgerMenu.addEventListener('click', function () {
                navLinks.classList.toggle('hidden');
                  if (bar1.classList.contains('rotate-0')) {
                    bar1.classList.remove('rotate-0');
                    bar1.classList.add('rotate-45', 'translate-y-2');

                    bar2.classList.remove('rotate-0');
                    bar2.classList.add('opacity-0');

                    bar3.classList.remove('rotate-0');
                    bar3.classList.add('-rotate-45', '-translate-y-2');
                } else {
                    bar1.classList.remove('rotate-45', 'translate-y-2');
                    bar1.classList.add('rotate-0');

                    bar2.classList.remove('opacity-0');
                    bar3.classList.remove('-rotate-45', '-translate-y-2');
                    bar3.classList.add('rotate-0');
                }
            });

        </script>
	</header>
}
"#
        .to_string(),
    };

    let footer = ProjectFile {
        filename: format!("footer.templ"),
        content: r#"package components

templ Footer(year string) {
	<footer class="bg-primary text-std p-3 text-center">
		<p class="text-sm">
			&copy; { year } GoApp. All rights reserved.
		</p>
	</footer>
}
"#
        .to_string(),
    };

    let components = ProjectDir {
        dirname: format!("components"),
        files: Some(vec![header, footer]),
        dirs: None,
    };

    let core = ProjectFile {
        filename: format!("core.templ"),
        content: format!(
            r#"package layouts

import "{0}/internal/models"
import "{0}/views/components"
import "strconv"

templ CoreHTML(site models.Site) {{
	<!DOCTYPE html>
	<html lang="en">
		<head>
			<meta name="viewport" content="width=device-width, initial-scale=1"/>
			<title>{{ site.AppName }} | {{ site.Title }}</title>
			<link rel="icon" href="/assets/images/favicon.ico" type="image/x-icon"/>
			<meta charset="utf-8"/>
			<meta name="viewport" content="width=device-width, initial-scale=1"/>
			<meta http-equiv="X-UA-Compatible" content="IE=edge"/>
			<meta name="description" content={{ site.Metatags.Description }}/>
			<meta name="keywords" content={{ site.Metatags.Keywords }}/>
			<meta name="author" content="Francecsco Michele Barranca"/>
			<meta name="robots" content="index, follow"/>
			<link rel="canonical" href="https://example.com"/>
			<script type="application/ld+json">
                {{
                    "@context": "http://schema.org",
                    "@type": "Organization",
                    "name": "GoApp",
                    "url": "https://example.com",
                    "logo": "https://example.com/assets/images/logo.webp",
                    "contactPoint": [
                        {{
                            "@type": "ContactPoint",
                            "telephone": "+1",
                            "contactType": ""
                        }}
                    ]
                }}
                </script>
			<script type="module" src="/assets/dist/index.js"></script>
			<link rel="stylesheet" href="/assets/dist/index.css"/>
		</head>
		<body class="h-full w-full flex flex-col justify-stretch items-stretch relative">
			<div id="indicator" class="htmx-indicator w-full h-screen absolute bottom-0 right-0 z-50 flex bg-slate-700 opacity-70 justify-center items-center"><div class="loader"></div></div>
			@components.Header()
			{{ children... }}
			@components.Footer(strconv.Itoa(site.Year))
		</body>
	</html>
}}
"#,
            import_str
        ),
    };

    let layouts = ProjectDir {
        dirname: format!("layouts"),
        files: Some(vec![core]),
        dirs: None,
    };

    let icons = ProjectDir {
        dirname: format!("icons"),
        files: None,
        dirs: None,
    };

    let client_error = ProjectFile {
        filename: format!("404.templ"),
        content: format!(
            r#"package views

import "{0}/internal/models"
import "{0}/views/layouts"

templ ClientError(site models.Site, err error) {{
	@layouts.CoreHTML(site) {{
		<main class="flex flex-col w-full justify-center items-center text-center h-[80vh]">
			<h1 class="rounded-sm text-2xl p-2 my-3 bg-red-800 shadow-xl text-white text-center">404 - Page not found</h1>
			<p class=" rounded-sm text-2xl p-2 my-3 bg-red-800 shadow-xl text-white w-full text-center tracking-wider">{{ err.Error() }}</p>
			<a class="rounded p-2 my-5 text-center italic shadow-2xl bg-green-800 text-white w-3/4 text-xl" href="/">Go back</a>
		</main>
	}}
}}
"#,
            import_str
        ),
    };

    let server_error = ProjectFile {
        filename: format!("500.templ"),
        content: format!(
            r#"package views

import "{0}/internal/models"
import "{0}/views/layouts"

templ ServerError(site models.Site, err error) {{
	@layouts.CoreHTML(site) {{
		<main class="flex flex-col w-full justify-center items-center text-center h-[80vh]">
			<h1 class="rounded-sm text-2xl p-2 my-3 bg-red-800 shadow-xl text-white text-center">500 - Server encoutered an error</h1>
			<p class=" rounded-sm text-2xl p-2 my-3 bg-red-800 shadow-xl text-white w-full text-center tracking-wider">{{ err.Error() }}</p>
			<a class="rounded p-2 my-5 text-center italic shadow-2xl bg-green-800 text-white w-3/4 text-xl" href="/">Go back</a>
		</main>
	}}
}}
"#,
            import_str
        ),
    };

    let index_html = ProjectFile {
        filename: format!("index.templ"),
        content: format!(
            r#"package views

import "{0}/internal/models"
import "{0}/views/layouts"

templ Index(site models.Site) {{
	@layouts.CoreHTML(site) {{
		<main class="flex flex-col w-full justify-center items-center min-h-[100vh]">
			<h1 class="text-2xl text-primary font-bold my-2">Hello GO+HTMX</h1>
		</main>
	}}
}}
"#,
            import_str
        ),
    };

    ProjectDir {
        dirname: format!("views"),
        dirs: Some(vec![components, layouts, icons]),
        files: Some(vec![client_error, server_error, index_html]),
    }
}

fn readme() -> ProjectFile {
    ProjectFile {
        filename: "README.md".to_string(),
        content: format!(""),
    }
}

fn air_toml() -> ProjectFile {
    ProjectFile {
        filename: ".air.toml".to_string(),
        content: r#"root = "."
testdata_dir = "testdata"
tmp_dir = "tmp"

[build]
  args_bin = []
  bin = "./tmp/main"
  cmd = "go build -o ./tmp/main ./cmd/server/*.go"
  delay = 1000
  exclude_dir = ["assets", "tmp", "vendor", "testdata", "client", "data"]
  exclude_file = []
  exclude_regex = ["_test.go"]
  exclude_unchanged = false
  follow_symlink = false
  full_bin = ""
  include_dir = []
  include_ext = ["go", "tpl", "tmpl", "html"]
  include_file = []
  kill_delay = "0s"
  log = "build-errors.log"
  poll = false
  poll_interval = 0
  post_cmd = []
  pre_cmd = []
  rerun = false
  rerun_delay = 500
  send_interrupt = false
  stop_on_error = false

[color]
  app = ""
  build = "yellow"
  main = "magenta"
  runner = "green"
  watcher = "cyan"

[log]
  main_only = false
  time = false

[misc]
  clean_on_exit = false

[screen]
  clear_on_rebuild = false
  keep_scroll = true
"#
        .to_string(),
    }
}

fn dockerignore() -> ProjectFile {
    ProjectFile {
        filename: ".dockerignore".to_string(),
        content: r#"build
assets
.vscode
*compose*
Dockerfile"#
            .to_string(),
    }
}

fn dotenvf(port: u32) -> ProjectFile {
    ProjectFile {
        filename: ".env".to_string(),
        content: format!(r#"PORT="{}""#, port),
    }
}

fn gitignore() -> ProjectFile {
    ProjectFile {
        filename: ".gitignore".to_string(),
        content: r#"node_modules
*.env
.env
data
vendor
node_modules
.gomon
.vscode
*.log
static/dist/*
build/postgres
data
*.pdf
build
*.tar
tmp
"#
        .to_string(),
    }
}

fn prodenv(port: u32) -> ProjectFile {
    ProjectFile {
        filename: ".prod.env".to_string(),
        content: format!(r#"PORT="{}""#, port),
    }
}

// fn dbenv(user: &str, database: &str, password: &str) -> ProjectFile {
//     ProjectFile {
//         filename: "db.env".to_string(),
//         content: format!(
//             r#"POSTGRES_USER="{}"
// POSTGRES_DB="{}"
// POSTGRES_PASSWORD="{}""#,
//             user, database, password
//         ),
//     }
// }

// fn proddbenv(user: &str, database: &str, password: &str) -> ProjectFile {
//     ProjectFile {
//         filename: "dbp.env".to_string(),
//         content: format!(
//             r#"POSTGRES_USER="{}"
// POSTGRES_DB="{}"
// POSTGRES_PASSWORD="{}""#,
//             user, database, password
//         ),
//     }
// }

fn dockercompose(project: &str, port: u32) -> ProjectFile {
    ProjectFile {
        filename: "docker-compose.yml".to_string(),
        content: format!(
            r#"version: "3.7"

services:
  {0}:
    container_name: {0}
    image: {0}
    labels:
      - traefik.http.routers.{0}.rule=Host(`{0}.example.com`)
      - traefik.http.routers.{0}.entrypoints=web,websecure
      - traefik.http.routers.{0}.service={0}
      - traefik.http.services.{0}.loadbalancer.server.port={1}
      - traefik.http.routers.{0}.tls=true
      - traefik.http.routers.{0}.tls.certresolver=le
      - traefik.port=80
    networks:
      - {0}net
      - proxy
    ports:
      - {1}:{1}

networks:
  proxy:
    external: true
  {0}net:
    driver: bridge
    external: false

volumes:
  {0}pgdata:
    driver: local
  {0}pgconf:
    driver: local
  {0}pglog:
    driver: local"#,
            project, port
        ),
    }
}

fn dockerfile(project: &str, port: u32) -> ProjectFile {
    ProjectFile {
        filename: "Dockerfile".to_string(),
        content: format!(
            r#"FROM golang:1.21.5-alpine3.18 AS build

RUN apk --no-cache add gcc g++ make git

WORKDIR /go/src/app

COPY . .

RUN go mod tidy

RUN mv .prod.env .env

RUN GOOS=linux go build -ldflags="-s -w" -o ./bin/{0} ./cmd/server/*.go

FROM alpine:3.18

RUN apk update && apk upgrade && apk --no-cache add ca-certificates

WORKDIR /go/bin

COPY --from=build /go/src/app/bin /go/bin
COPY --from=build /go/src/app/.env /go/bin/
COPY --from=build /go/src/app/data /go/bin/data
COPY --from=build /go/src/app/static /go/bin/static

EXPOSE {1}

ENTRYPOINT /go/bin/{0} --port {1}"#,
            project, port
        ),
    }
}
