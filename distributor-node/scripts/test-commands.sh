#!/usr/bin/env bash

SCRIPT_PATH="$(dirname "${BASH_SOURCE[0]}")"
cd $SCRIPT_PATH

export AUTO_CONFIRM=true
export CONFIG_PATH="../config.yml"
CLI=../bin/run

${CLI} leader:set-buckets-per-bag-limit -l 10
FAMILY_ID=`${CLI} leader:create-bucket-family ${CONFIG}`
BUCKET_ID=`${CLI} leader:create-bucket -f ${FAMILY_ID} -a yes`
${CLI} leader:update-bag -b static:council -f ${FAMILY_ID} -a ${BUCKET_ID}
${CLI} leader:update-bucket-status -f ${FAMILY_ID} -B ${BUCKET_ID}  --acceptingBags yes
${CLI} leader:update-bucket-mode -f ${FAMILY_ID} -B ${BUCKET_ID} --mode on
${CLI} leader:update-dynamic-bag-policy -t Member -p ${FAMILY_ID}:5
${CLI} leader:update-dynamic-bag-policy -t Member
${CLI} leader:delete-bucket -f ${FAMILY_ID} -B ${BUCKET_ID}
${CLI} leader:delete-bucket-family -f ${FAMILY_ID}
