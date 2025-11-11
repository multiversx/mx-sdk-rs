if [[ -n "framework/meta" ]];
then
    cargo install --path framework/meta && echo "Installed sc-meta from path"
elif [[ -z "" ]];
then
    cargo install multiversx-sc-meta --locked --force && echo "Installed latest sc-meta version"
else
    cargo install multiversx-sc-meta --version  --locked --force && echo "Installed sc-meta version "
fi

if [[ -z "v5.0.0" ]];
then
    sc-meta install mx-scenario-go && echo "Installed latest mx-scenario-go version"
else
    sc-meta install mx-scenario-go --tag v5.0.0 && echo "Installed mx-scenario-go version v5.0.0"
fi

# if [[ -n "framework/meta" ]];
# then
#     cargo install --path framework/meta && echo "Installed sc-meta from path updatedd"
# elif [[ -z "" ]];
# then
#     cargo install multiversx-sc-meta --locked && echo "Installed latest sc-meta version updated"
# else
#     cargo install multiversx-sc-meta --version --locked
# fi


# if [[ -z "v5.0.0" ]];
# then
#     sc-meta install mx-scenario-go && echo "Installed latest mx-scenario-go version"
# else
#     sc-meta install mx-scenario-go --tag v5.0.0 && echo "Installed mx-scenario-go version v5.0.0"
# fi
