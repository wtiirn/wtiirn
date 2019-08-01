#!/bin/sh
 curl --header "Content-Type: text/xml;charset=UTF-8" https://ws-shc.qc.dfo-mpo.gc.ca/predictions --header "SOAPAction:search" --data @tides_gc_ca_predictions_search.xml | tidy -xml -i

