from fastapi import FastAPI
from fastapi.responses import HTMLResponse
from starlette.middleware.cors import CORSMiddleware

from app.api.v1.api import api_router
from app.core.config import settings

app = FastAPI(
    title="Exyle.io API",
    openapi_url=f"{settings.API_V1_STR}/openapi.json",
    docs_url=None,  # disable swagger
    redoc_url=None,  # disable redoc
)

app.add_middleware(
    CORSMiddleware,
    allow_origins=["*"],
    allow_methods=["*"],
    allow_headers=["*"],
    allow_credentials=True,
)

app.include_router(api_router, prefix=settings.API_V1_STR)


@app.get("/rapidoc", response_class=HTMLResponse, include_in_schema=False)
async def rapidoc():
    return f"""
        <!doctype html>
        <html>
            <head>
                <meta charset="utf-8">
                <title>Exyle.io API | Documentation</title>
                <script 
                    type="module" 
                    src="https://unpkg.com/rapidoc/dist/rapidoc-min.js"
                ></script>
            </head>
            <body>
                <rapi-doc
                    id="rapidoc"
                    spec-url="{app.openapi_url}"
                    sort-tags="true"
                    sort-endpoints-by="path"
                    fill-request-fields-with-example="true"
                    theme="dark"
                    load-fonts="true"
                    font-size="default"
                    use-path-in-nav-bar="false"
                    nav-bg-image-repeat="repeat"
                    nav-item-spacing="default"
                    layout="row"
                    render-style="view"
                    on-nav-tag-click="expand-collapse"
                    schema-style="tree"
                    schema-expand-level="999"
                    schema-description-expanded="false"
                    schema-hide-read-only="always"
                    schema-hide-write-only="always"
                    default-schema-tab="model"
                    response-area-height="300px"
                    show-info="true"
                    info-description-headings-in-navbar="false"
                    show-components="false"
                    show-header="true"
                    allow-authentication="true"
                    allow-spec-url-load="false"
                    allow-spec-file-load="false"
                    allow-search="true"
                    allow-try="true"
                    allow-server-selection="true"
                    allow-schema-description-expand-toggle="true"
                    api-key-location="header"
                    fetch-credentials="same-origin">
                </rapi-doc>
            </body> 
        </html>
    """
